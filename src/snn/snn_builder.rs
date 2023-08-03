use crate::snn::neuron::Neuron;
#[derive(Debug, Clone)]
pub struct SnnParams<N: Neuron+ Clone>{
    neurons: Vec<Vec<N>>,
    extra_weights: Vec<Vec<Vec<f64>>>,
    intra_weights: Vec<Vec<Vec<f64>>>,
}
pub struct SnnBuilder<N: Neuron+ Clone, const INPUT_DIM: usize>{
    params: SnnParams<N>
}

impl <N: Neuron+ Clone,  const INPUT_DIM: usize> SnnBuilder<N, INPUT_DIM> {
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

    pub fn add_layer(&mut self) -> &mut SnnBuilder<N, { INPUT_DIM }> {
        self
    }
    pub fn add_weight<const NUM_NEURONS: usize>(&mut self, weights:[[f64; INPUT_DIM]; NUM_NEURONS]) -> &mut SnnBuilder<N, { INPUT_DIM }> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &weights{
            for n in n_weight{
                //todo!()
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.extra_weights.push(new_weights);
        self
    }
    pub fn add_neurons<const NUM_NEURONS: usize>(&mut self, neurons: [N; NUM_NEURONS]) -> &mut SnnBuilder<N, { INPUT_DIM }> {
        self.params.neurons.push(Vec::from(neurons));
        self
    }
    pub fn add_intra_weights<const NUM_NEURONS: usize>(&mut self, intra_weights: [[f64; NUM_NEURONS]; NUM_NEURONS]) -> &mut SnnBuilder<N, { INPUT_DIM }> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &intra_weights{
            for n in n_weight{
                //todo!()
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.intra_weights.push(new_weights);
        self
    }
}