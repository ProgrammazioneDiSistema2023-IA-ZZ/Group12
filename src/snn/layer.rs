use std::sync::mpsc::{Receiver, Sender};
use crate::snn::Evento;
use crate::snn::neuron::Neuron;

pub struct Layer<N: Neuron+Clone+'static>{
    neurons: Vec<N>,
    weights: Vec<Vec<f64>>,
    intra_weights: Vec<Vec<f64>>,
    prev_output: Vec<u8>,
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
            prev_output: vec![0; len]
        }
    }

    pub fn process(&mut self, layer_input_rc: Receiver<Evento>, layer_output_tx: Sender<Evento>){

        /**Inizializiamo il layer **/
        self.init_layer();

        /** Prendiamo l'output del layer precedente **/
        if let Ok(input_spike) = layer_input_rc.recv() {
            let instant = input_spike.ts;
            let mut output_spikes = Vec::<u8>::with_capacity(self.neurons.len());
            let mut at_least_one_spike = false;

            /** Calcoliamo i pesi sia intra-layer che extra-layer, e calcoliamo
                l'output
            **/

            for (index, neuron) in self.neurons.iter_mut().enumerate(){
                let mut intra_weights_sum = 0f64;
                let mut extra_weights_sum = 0f64;

                todo!();

                /** Calcolo il potenziale di membrana e l'output del neurone **/
                let neuron_spike = neuron.update_v_mem(instant,intra_weights_sum, extra_weights_sum);
                output_spikes.push(neuron_spike);


            }

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
        }
    }
}
