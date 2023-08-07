use crate::snn::neuron::Neuron;

#[derive(Debug, Clone)]
pub struct SnnParams<N: Neuron+ Clone>{
    neurons: Vec<Vec<N>>,
    extra_weights: Vec<Vec<Vec<f64>>>,
    intra_weights: Vec<Vec<Vec<f64>>>,
}
pub struct SnnBuilder<N: Neuron+ Clone>{
    params: SnnParams<N>
}

impl <N: Neuron+ Clone> SnnBuilder<N> {
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

    pub fn add_weights(){

    }
}