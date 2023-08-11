use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
use crate::snn::snn::SNN;

#[derive(Debug, Clone)]
pub struct SnnParams<N: Neuron+ Clone+Debug+'static>{
    neurons: Vec<Vec<N>>,
    extra_weights: Vec<Vec<Vec<f64>>>,
    intra_weights: Vec<Vec<Vec<f64>>>,
}
#[derive(Debug, Clone)]
pub struct SnnBuilder<N: Neuron+ Clone+Debug+'static>{
    params: SnnParams<N>
}

impl <N: Neuron+ Clone+Debug> SnnBuilder<N> {

    pub fn new()->Self{
        Self {
            params: SnnParams {
                neurons: vec![],
                extra_weights: vec![],
                intra_weights: vec![],
            }
        }
    }
    pub fn get_params(&self) -> SnnParams<N> {
        self.params.clone()
    }

    /** Per ora serve solo logicamente */
    pub fn add_layer(&mut self) -> &mut SnnBuilder<N> {
        self
    }


    pub fn add_weight<const NUM_NEURONS: usize, const INPUT_DIM: usize >(&mut self, weights:[[f64; INPUT_DIM]; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &weights{
            for w in n_weight{
                if w < &0.0{
                    panic!("Pesi devono essere positivi!");
                }
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.extra_weights.push(new_weights);
        self
    }
    pub fn add_neurons<const NUM_NEURONS: usize>(&mut self, neurons: [N; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        self.params.neurons.push(Vec::from(neurons));
        self
    }
    pub fn add_intra_weights<const NUM_NEURONS: usize>(&mut self, intra_weights: [[f64; NUM_NEURONS]; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &intra_weights{
            for w in n_weight{
                if w>&0f64{
                    panic!("Pesi nello stesso layer devono essere negativi!");
                }
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.intra_weights.push(new_weights);
        self
    }

    pub fn build<const INPUT_DIM: usize, const OUTPUT_DIM:usize>(self)-> SNN<N, { INPUT_DIM }, { OUTPUT_DIM }>{
        if self.params.extra_weights.len() != self.params.neurons.len() || self.params.intra_weights.len() != self.params.neurons.len(){
            panic!("Wrong number bewteen layers!")
        }
        //Eventualmente fare check su #pesi

        let mut layers: Vec<Arc<Mutex<Layer<N>>>> = Vec::new();
        let mut n_iter = self.params.neurons.into_iter();
        let mut extra_iter = self.params.extra_weights.into_iter();
        let mut intra_iter = self.params.intra_weights.into_iter();

        while let Some(layer) = n_iter.next() {
            let new_extra_iter = extra_iter.next().unwrap();
            let new_intra_iter = intra_iter.next().unwrap();

            /* create and save the new layer */
            let new_layer = Layer::new(layer, new_extra_iter, new_intra_iter);
            layers.push(Arc::new(Mutex::new(new_layer)));
        }
        SNN::<N, {INPUT_DIM }, { OUTPUT_DIM }>::new(layers)
    }


}