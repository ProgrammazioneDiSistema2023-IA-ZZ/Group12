use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use crate::snn::Evento;
use crate::snn::neuron::Neuron;
use crate::snn::error_handling;
use rand::Rng;
use crate::snn::components::{Adder, Multiplier};

/// Struttura che rappresenta un errore transitorio bit-flip su un bit di un componente
struct TransientError{
/// Indice del neurone su cui è presente l'errore
    neuron:usize,
/// Componente su cui è presente l'errore
    component:i32,
/// Posizione del bit del componente su cui effettuare il flip
    position:u8,
/// Istante di tempo in cui si verifica l'errore
    time: u64,

    input_errors: (i32,i32)
}
impl TransientError {
    pub fn new(neuron: usize, component: i32, position: u8, time: u64, input_errors: (i32, i32)) -> Self {
        Self { neuron, component, position, time, input_errors }
    }
}
/// Layer della rete neurale
pub struct Layer<N: Neuron+Clone+'static>{
/// Vettore di neuroni nel layer
    neurons: Vec<N>,
/// Vettore di vettori di pesi tra ciascun neurone del layer e i neuroni del layer precedente
    weights: Vec<Vec<f64>>,
/// Vettore di vettori di pesi tra ciascun neurone del layer
/// e gli altri neuroni del layer stesso
    intra_weights: Vec<Vec<f64>>,
/// Impulsi di output del layer nell'istante precendete
    prev_output: Vec<u8>,
/// Eventuale errore transitorio su uno dei componenti del layer
    error: Option<TransientError>
}

impl<N: Neuron+ Clone+'static> Layer<N> {
    /*** Getters ***/
    pub fn neurons(&self) -> &Vec<N> {
        &self.neurons
    }
    pub fn weights(&self) -> &Vec<Vec<f64>> {
        &self.weights
    }
    pub fn intra_weights(&self) -> &Vec<Vec<f64>> {
        &self.intra_weights
    }
    pub fn prev_output(&self) -> &Vec<u8> {
        &self.prev_output
    }
/// Ritorna un nuovo layer per una rete neurale
/// # Argomenti
/// * `neurons` - vettore di neuroni che costituiscono il layer
/// * `weights` - pesi con il layer precedente
/// * `intra_weights` - pesi interni tra i neuroni del layer stesso
/// # Valori predefiniti
/// * `prev_output` - output precedente del layer settato con valori a 0
/// * `error` - nessun errore transitorio (Option::None)
    pub fn new(neurons: Vec<N>, weights: Vec<Vec<f64>>, intra_weights: Vec<Vec<f64>>)->Self{
        let len= neurons.len();
        Self{
            neurons,
            weights,
            intra_weights,
            prev_output: vec![0; len],
            error:None
        }
    }
/// Setta un errore transitorio bit-flip su uno dei componenti del layer
/// # Argomenti
/// * `neuron` - indice del neurone sul cui relativo componente è presente l'errore
/// * `componet` - componente su cui avviene l'errore. Può avere valore:
///     * `0` -> potenziale di soglia
///     * `1` -> potenziale di membrana
///     * `2` -> uno dei pesi esterni, verso il neurone specificato
///     * `3` -> uno dei pesi interni, dal neurone specificato
/// * `position` - posizione del bit affetto da errore
/// * `time` - istante di tempo in cui si verifica l'errore
    pub fn set_transient_error(&mut self, neuron: usize, component: i32, position: u8, time: u64, input_errors: (i32, i32)){
        self.error=Some(TransientError::new(neuron,component,position,time, input_errors))
    }
/// Genera a caso l'indice del peso su cui applicare l'errore
    fn random_w_index(w:&Vec<f64>)->usize{
        let mut rng = rand::thread_rng();
        rng.gen_range(0..w.len())
    }
/// Funzione per controllare la presenza di un errore transitorio nel layer
/// e se questo avviene nell'istante *current_instant* specificato
    fn check_transient_error(&mut self, current_instant: u64, adder: &mut Adder,  mult: &mut Multiplier) ->Option<(Adder, Multiplier)>{
        if self.error.is_none() { return None; }
        let transient_error= self.error.as_ref().unwrap();
        /* controllo sull'istante di tempo*/
        if transient_error.time !=current_instant { return None; }

        let n=&mut self.neurons[transient_error.neuron];
        let position=transient_error.position;
        match transient_error.component {
            //Threshold
            0=>{error_handling::threshold_fault(n,2,position); return None;},
            //Membrane
            1=>{error_handling::membrane_fault(n,2,position); return  None;},
            //Extra
            2=>{
                let w= &mut self.weights[transient_error.neuron];
                let index=Layer::<N>::random_w_index(w);
                error_handling::weight_fault(&mut w[index],2, position);
                return  None;
            },
            //Intra
            3=>{
                let w= &mut self.intra_weights[transient_error.neuron];
                let index=Layer::<N>::random_w_index(w);
                error_handling::weight_fault(&mut w[index],2, position);
                return  None;
            },
            4=>{
                adder.set_params(2, position);
                return Some((*adder, *mult))
            },
            5=>{
                adder.set_params_input(2, position, transient_error.input_errors.0,transient_error.input_errors.1);
                return Some((*adder, *mult))
            },
            6=>{
                mult.set_params(2, position);
                return Some((*adder, *mult))
            },
            7=>{
                mult.set_params_input(2, position, transient_error.input_errors.0,transient_error.input_errors.1);
                return Some((*adder, *mult))
            },
            _=>{},
        }
        return Some((*adder, *mult));
    }
/// Funzione per processare gli impulsi in input al layer
/// # Argomenti
/// * `layer_input_rc` - **Receiver** del channel con il layer precedente, attende la ricezione dell'Evento rappresentante gli impulsi in input
/// * `layer_output_tx` - **Sender** del channel con il layer successivo, invia l'Evento rappresentante gli impulsi di output
    pub fn process(&mut self, adder: Adder, multiplier:  Multiplier, layer_input_rc: Receiver<Evento>, layer_output_tx: Sender<Evento>){

        /* Prendiamo l'output del layer precedente */
        while let Ok(input_spike) = layer_input_rc.recv() {
            let mut local_adder=  adder;
            let mut local_mult = multiplier;

            let instant = input_spike.ts;
            let mut output_spikes = Vec::<u8>::with_capacity(self.neurons.len());
            /* controlliamo che non vi sia un transient bit-flip in questo determinato istante */
            let check_res =self.check_transient_error(instant, &mut adder.clone(), &mut multiplier.clone());
            match check_res{
                None=>{}
                Some((adder_new, mult_new))=>{
                    local_adder = adder_new;
                    local_mult = mult_new;
                }
            }

            /* Processiamo l'input per ogni neurone nel layer */
            for (n_index, neuron) in self.neurons.iter_mut().enumerate(){
                let mut intra_weights_sum = 0f64;
                let mut extra_weights_sum = 0f64;
                /* Somma pesata degli ingressi al neurone in base agli extra-weights */
                for (w_index, weight) in self.weights[n_index].iter().enumerate(){
                    if input_spike.spikes[w_index] != 0 {
                        extra_weights_sum += weight;
                    }
                }
                /* Somma pesata degli effetti dell'output precedente del neurone,
                    dipendente dagli intra-weights */
                for (i_index, intra) in self.intra_weights[n_index].iter().enumerate(){
                    if i_index != n_index && self.prev_output[i_index] != 0{
                        intra_weights_sum+= intra;
                    }
                }
                /* Calcoliamo il potenziale di membrana e l'output del neurone */
                let neuron_spike = neuron.update_v_mem(instant,intra_weights_sum, extra_weights_sum, local_adder.clone(), local_mult.clone());
                /* Salvataggio dell'output del neurone nel vettore contenente l'output totale del layer */
                output_spikes.push(neuron_spike);
            }
            /* Salvataggio dell'output per il prossimo istante */
            self.prev_output=output_spikes.clone();
            /* Creazione dell'Evento contenente l'output da inviare al prossimo layer */
            let output_spike = Evento::new(instant, output_spikes);

            /* Mandiamo l'output al prossimo layer */
            layer_output_tx.send(output_spike)
                .expect(&format!("ERROR: sending spike event at t={}", instant));
        }
    }
/// Inizializzazione del layer; pulizia del vettore prev_output e re-inizializzazione di tutti i neuroni
    pub fn init_layer(&mut self){
        self.prev_output.clear();
        self.neurons.iter_mut().for_each(|neuron| neuron.init_neuron());
    }
}

impl <N: Neuron+Clone+'static> Clone for Layer<N>{
    fn clone(&self) -> Self {
        Self{
            neurons: self.neurons.clone(),
            weights: self.weights.clone(),
            intra_weights: self.intra_weights.clone(),
            prev_output: self.prev_output.clone(),
            error: None,
        }
    }
}
