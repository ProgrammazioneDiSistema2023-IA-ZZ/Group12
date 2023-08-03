use crate::snn::neuron::Neuron;

pub struct Layer<N: Neuron+ Clone>{
    neurons: Vec<N>,
    weights: Vec<Vec<f64>>,
    intra_weights: Vec<Vec<f64>>,
    prev_output: Vec<u8>,
}


impl<N: Neuron+ Clone> Layer<N> {
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
        Self{
            neurons,
            weights,
            intra_weights,
            prev_output: vec![0; neurons.len()]
        }
    }

    pub fn process(){
        todo!();
    }

    pub fn init_layer(&mut self){
        self.prev_output.clear();
        self.neurons.iter_mut().for_each(|neuron| neuron.init_neuron());
    }
}

impl <N: Neuron+Clone> Clone for Layer<N>{
    fn clone(&self) -> Self {
        Self{
            neurons: self.neurons.clone(),
            weights: self.weights.clone(),
            intra_weights: self.intra_weights.clone(),
            prev_output: self.prev_output.clone(),
        }
    }
}
