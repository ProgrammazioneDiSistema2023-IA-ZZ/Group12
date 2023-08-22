use std::fmt::Debug;
use std::sync::mpsc::{Receiver, Sender};
use crate::snn::Evento;
use crate::snn::neuron::Neuron;
use crate::snn::error_handling;
use rand::Rng;

pub struct TransientError{
    neuron:usize,
    component:i32,
    position:u8,
    time: u64,
}
impl TransientError {
    pub fn new( neuron: usize, component: i32, position: u8, time: u64) -> Self {
        Self { neuron, component, position, time }
    }
}

pub struct Layer<N: Neuron+Clone+'static>{
    neurons: Vec<N>, /* neuroni del layer */
    weights: Vec<Vec<f64>>, /* pesi tra i neuroni di questo layer con quelli del layer precedente */
    intra_weights: Vec<Vec<f64>>, /* pesi tra i neuroni dello stesso layer */
    prev_output: Vec<u8>, /* impulsi di output al precedente istante */
    error: Option<TransientError>
}

impl<N: Neuron+ Clone+'static> Layer<N> {
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
    pub fn set_transient_error(&mut self, neuron: usize, component: i32, position: u8, time: u64){
        self.error=Some(TransientError::new(neuron,component,position,time))
    }
    fn random_w_index(w:&Vec<f64>)->usize{
        let mut rng = rand::thread_rng();
        rng.gen_range(0..w.len())
    }
    fn check_transient_error(&mut self, current_instant: u64){
        if self.error.is_none() { return; }
        let transient_error= self.error.as_ref().unwrap();
        if transient_error.time !=current_instant { return; }
        println!("Transient error at time: {}, neuron: {}, component: {}",
                 current_instant, transient_error.neuron, transient_error.component);
        let n=&mut self.neurons[transient_error.neuron];
        let position=transient_error.position;
        match transient_error.component {
            //Threshold
            0=>{error_handling::threshold_fault(n,2,position);},
            //Membrane
            1=>{error_handling::membrane_fault(n,2,position);},
            //Extra
            2=>{
                let w= &mut self.weights[transient_error.neuron];
                let index=Layer::<N>::random_w_index(w);
                error_handling::weight_fault(&mut w[index],2, position);
            },
            //Intra
            3=>{
                let w= &mut self.intra_weights[transient_error.neuron];
                let index=Layer::<N>::random_w_index(w);
                error_handling::weight_fault(&mut w[index],2, position);
            },
            _=>{},
        }
        return;
    }
    pub fn process(&mut self, layer_input_rc: Receiver<Evento>, layer_output_tx: Sender<Evento>){

        /** Prendiamo l'output del layer precedente **/
        while let Ok(input_spike) = layer_input_rc.recv() {
            let instant = input_spike.ts;
            let mut output_spikes = Vec::<u8>::with_capacity(self.neurons.len());
            let mut at_least_one_spike = false;
            // controlliamo che non vi sia un transient bit-flip
            self.check_transient_error(instant);
            /** Calcoliamo i pesi sia intra-layer che extra-layer, e calcoliamo
                l'output
            **/

            for (n_index, neuron) in self.neurons.iter_mut().enumerate(){
                let mut intra_weights_sum = 0f64;
                let mut extra_weights_sum = 0f64;

                for (w_index, weight) in self.weights[n_index].iter().enumerate(){
                    if input_spike.spikes[w_index] != 0 {
                        extra_weights_sum += weight;
                    }
                }
                for (i_index, intra) in self.intra_weights[n_index].iter().enumerate(){
                    if i_index != n_index && self.prev_output[i_index] != 0{
                        intra_weights_sum+= intra;
                    }
                }
                /** Calcolo il potenziale di membrana e l'output del neurone **/
                let neuron_spike = neuron.update_v_mem(instant,intra_weights_sum, extra_weights_sum);
                output_spikes.push(neuron_spike);


            }
            self.prev_output=output_spikes.clone();
            /** Creiamo l'output del prossimo layer **/
            let output_spike = Evento::new(instant, output_spikes);

            /** Mandiamo l'output al prossimo layer **/
            layer_output_tx.send(output_spike)
                .expect(&format!("ERROR: sending spike event at t={}", instant));
        }


    }

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
