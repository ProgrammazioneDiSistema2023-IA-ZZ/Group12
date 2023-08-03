use crate::models::lifneuron::LIFNeuron;

mod models;
mod snn;


fn main(){
    LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0);
    println!("LIF Neuron Created!")

}