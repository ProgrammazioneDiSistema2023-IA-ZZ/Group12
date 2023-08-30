use std::slice::IterMut;
use std::sync::{Arc, Mutex};
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
use crate::snn::Evento;
use crate::snn::processor::Processor;
use rand::Rng;
use crate::snn::components::{Adder, Multiplier};


/// Struttura che rappresenta la rete neurale
/// # Campi
/// * `layers` - vettore dei layer che costituiscono la rete
/// * `transient_error` - parametro opzionale temporaneo che contiene le informazioni relative a un possibile
/// errore transitorio, in attesa che venga selezionato un istante casuale
/// # Tipi e costanti
/// * `N` - tipo generico per rappresentare un Neurone
/// * `SNN_INPUT_DIM` - dimensione dell'input della rete
/// * `SNN_OUTPUT_DIM` - dimensione dell'output della rete
pub struct SNN<N: Neuron + Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM: usize>{
    layers: Vec<Arc<Mutex<Layer<N>>>>,
    transient_error: Option<(usize, usize,i32, u8)>,//layer, neuron, component, position
    adder: Adder,
    multiplier: Multiplier
}

impl <N:Neuron + Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM: usize>
    SNN<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM> {
    pub fn new(layers: Vec<Arc<Mutex<Layer<N>>>>, transient_error: Option<(usize, usize, i32, u8)>, adder: Adder, multiplier: Multiplier) -> Self {
        Self {
            layers,
            transient_error,
            adder,
            multiplier,
        }
    }

    pub fn layers(&self) -> &Vec<Arc<Mutex<Layer<N>>>> {
        &self.layers
    }

/// Funzione per processare gli impulsi in ingresso. Ritorna gli impulsi in uscita dalla rete.
/// Se il parametro opzionale `transient_error` presenta dei valori, viene selezionato
/// un istante casuale dipendente da `SPIKES_DURATION` in cui si presenterà l'errore transitorio.
/// # Argomenti
/// * `input_spikes` - vettore di vettori di impulsi in istanti successivi
/// # Costanti
/// * `SPIKES_DURATION` - durata dell'ingresso, i.e. numero di istanti in cui viene fornito un input
/// * `SNN_INPUT_DIM` - dimensione dell'input della rete
/// * `SNN_OUTPUT_DIM` - dimensione dell'output della rete
/// # Esempio di utilizzo
/// ```
///     snn.process(&[[0,1,1], [1,0,1]])
/// ```
/// implica `SNN_INPUT_DIM = 3` e `SPIKES_DURATION = 2`, i.e.:
/// - All'istante `0`, l'ingresso della rete vale `[0,1,1]`
/// - All'istante `1`, l'ingresso della rete vale `[1,0,1]`
    pub fn process<const SPIKES_DURATION: usize>(&mut self, input_spikes: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION])
                                                 -> [[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION] {
        /* trasformiamo l'input in Eventi */
        let input_events = SNN::<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>::spikes_to_events(input_spikes);
        if self.transient_error.is_some(){
            let (layer, neuron, component, position)=self.transient_error.unwrap();
            let mut rng=rand::thread_rng();
            let random_instant:u64=rng.gen_range(0..SPIKES_DURATION) as u64;
            /* settiamo l'errore transitorio sul layer corrispontente */
            self.layers[layer].lock().unwrap().set_transient_error(neuron,component,position,random_instant);
        }
        let processor = Processor {};
        let adder = self.adder.clone();
        let mult = self.multiplier.clone();
        let output_events = processor.process_events(self, input_events, adder, mult);

        /* trasformiamo gli Eventi di output in vettori di segnali, in modo tale che
            il valore di ritorno sia coerente con l'argomento in ingresso della funzione */
        let output_spikes:[[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION]
            = SNN::<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>::spikes_from_events(output_events);

        output_spikes
    }


/// Trasforma i vettori di segnali in ingresso in Eventi di impulsi che contegano le stesse informazioni.
/// Controlla inoltre che i valori passati rappresentino effettivamente dei segnali, i.e. siano `0` o `1`
    fn spikes_to_events<const SPIKES_DURATION: usize>
        (spikes_matrix: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION]) -> Vec<Evento> {

        let mut eventi = Vec::<Evento>::new();
        for ts in 0..SPIKES_DURATION {
            if spikes_matrix[ts].iter().any(|&s| s!=0 && s!=1){
                panic!("Error: input spike must be 0 or 1 ");
            }
            let ts_spikes=spikes_matrix[ts].to_vec();
            let evento_ts=Evento::new(ts as u64, ts_spikes);
            eventi.push(evento_ts);
        }

        eventi
    }

/// Trasforma gli Eventi in vettori di segnali
    fn spikes_from_events<const SPIKES_DURATION: usize>
        (eventi: Vec<Evento>) -> [[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION] {

        let mut raw_matrix = [[0u8; SNN_OUTPUT_DIM]; SPIKES_DURATION];

        for evento in eventi {
            if evento.spikes.len() != SNN_OUTPUT_DIM{
                panic!("Error: spikes in the event should equal the output dimension")
            }
            for (n_index, spike) in evento.spikes.into_iter().enumerate() {
                raw_matrix[evento.ts as usize][n_index] = spike;
            }
        }

        raw_matrix
    }

}

impl<'a, N: Neuron+Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM : usize > IntoIterator for &'a mut SNN<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>{
    type Item = &'a mut Arc<Mutex<Layer<N>>>;
    type IntoIter = IterMut<'a, Arc<Mutex<Layer<N>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.iter_mut()
    }
}