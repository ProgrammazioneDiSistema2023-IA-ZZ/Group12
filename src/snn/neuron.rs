pub trait Neuron: Send{

    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64)->u8;

    fn init_neuron(&mut self);
    fn set_membrane_error(&mut self, error_type:u8, position:u8);

    fn get_th(&self) -> f64;

    fn set_th(&mut self, new_th: f64);

    fn get_mem(&self) -> f64;

    fn set_mem(&mut self, new_mem: f64);

}