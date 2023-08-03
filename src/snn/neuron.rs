pub trait Neuron: Send{

    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64)->u8;

    fn init_neuron(&mut self);

}