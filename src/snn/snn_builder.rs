use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
use crate::snn::snn::SNN;
use rand::Rng;
use crate::snn::error_handling;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(EnumIter)]
pub enum ErrorComponent{
    ThresholdError,
    IntraWeightsError,
    ExtraWeightsError,
    MembraneError
}

#[derive(Debug, Clone)]
pub struct SnnParams<N: Neuron+ Clone+Debug+'static>{
    neurons: Vec<Vec<N>>,
    extra_weights: Vec<Vec<Vec<f64>>>,
    intra_weights: Vec<Vec<Vec<f64>>>,
}
#[derive(Debug, Clone)]
pub struct SnnBuilder<N: Neuron+Clone+Debug+'static>{
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

    fn choose_neuron(neurons: &Vec<Vec<N>>) -> (usize,usize){
        let mut rng = rand::thread_rng();

        let n_layers = rng.gen_range(0..neurons.len());
        let n_neuron = rng.gen_range(0..neurons[n_layers].len());

        (n_layers,n_neuron)

    }

    fn choose_weights(weights: &Vec<Vec<Vec<f64>>>) -> (usize,usize,usize){
        let mut rng = rand::thread_rng();

        let n_layers = rng.gen_range(0..weights.len());
        let n_weights = rng.gen_range(0..weights[n_layers].len());
        let n_weight = rng.gen_range(0..weights[n_layers][n_weights].len());

        (n_layers,n_weights, n_weight)

    }

    pub fn build<const INPUT_DIM: usize, const OUTPUT_DIM:usize>(&mut self  , components: &Vec<i32>, errorType: i32)-> SNN<N, { INPUT_DIM }, { OUTPUT_DIM }>{
        if self.params.extra_weights.len() != self.params.neurons.len() || self.params.intra_weights.len() != self.params.neurons.len(){
            panic!("Wrong number bewteen layers!")
        }
        if components.len() == 0 {

        }
        //Eventualmente fare check su #pesi

        let mut layers: Vec<Arc<Mutex<Layer<N>>>> = Vec::new();
        let mut n_iter = self.params.neurons.clone().into_iter();
        let mut extra_iter = self.params.extra_weights.clone().into_iter();
        let mut intra_iter = self.params.intra_weights.clone().into_iter();

        /** Generazione dell'errore sul componente casuale **/
        let mut rng = rand::thread_rng();
        /** Se il vettore è vuoto allora non ci sono componenti da verificare **/
        let mut component = -1;
        if components.len() != 0 {
            let component_index = rng.gen_range(0..components.len());
            component = components[component_index];
        }
        let (layer_index, neuron_index) = SnnBuilder::choose_neuron(&self.params.neurons.clone());
        let (layer_index_weight, index_weights, index_weight) = SnnBuilder::<N>::choose_weights(&self.params.extra_weights.clone());
        let (layer_intra_index_weight, index_intra_weights, index_intra_weight) = SnnBuilder::<N>::choose_weights(&self.params.intra_weights.clone());



        /** Generazione componente guasto **/
        /** Possibili componenti guasti
         -- Soglia
         -- Pesi
         -- Potenziali
        **/
        match component {
            0 => error_handling::threshold_fault(&mut self.params.neurons[layer_index][neuron_index], errorType),
            1 => error_handling::membrane_fault(&mut self.params.neurons[layer_index][neuron_index], errorType),
            2 => error_handling::extra_weights_fault(&mut self.params.extra_weights[layer_index_weight][index_weights][index_weight], errorType),
            3 => error_handling::extra_weights_fault(&mut self.params.intra_weights[layer_intra_index_weight][index_intra_weights][index_intra_weight], errorType),
            _ =>{},
        }


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