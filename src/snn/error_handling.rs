use rand::Rng;
use crate::snn;
use crate::snn::error_handling::ErrorType::{Flip, Stuck0, Stuck1};
use crate::snn::neuron::Neuron;

pub enum ErrorType{
    Stuck0,
    Stuck1,
    Flip,
    None
}

fn embed_error(variable:f64, error:ErrorType)->f64{
    let mut bit_value:u64=variable.to_bits();
    let mut rng = rand::thread_rng();
    let position: u32 = rng.gen_range(0..64);
    bit_value=match error {
        ErrorType::None => bit_value,
        ErrorType::Stuck0 => unset_bit(bit_value, position),
        ErrorType::Stuck1 => set_bit(bit_value, position),
        ErrorType::Flip => flip_bit(bit_value, position),
    };

    f64::from_bits(bit_value)
}
/**
Setta a 1 il bit in posizione *position* di *value*
 */
fn set_bit(value:u64, position:u32)->u64{
    /* maschera del tipo 0001000 */
    let bit_mask=1u64<<position;
    value | bit_mask
}
fn unset_bit(value:u64, position:u32)->u64{
    /* maschera del tipo 1110111 */
    let bit_mask=!(1u64<<position);
    value & bit_mask
}
fn flip_bit(value:u64, position:u32)->u64{
    /* maschera del tipo 0001000 */
    let bit_mask=1u64<<position;
    value ^ bit_mask
}

pub fn threshold_fault<N: Neuron+Clone+'static>(neuron: &mut N){
    println!("Old Threshold -> {}", neuron.get_th());
    let new_threshold = embed_error(neuron.get_th(), Stuck1);

    neuron.set_th(new_threshold);
    println!("New Threshold -> {}", neuron.get_th());

}

pub fn membrane_fault<N: Neuron+Clone+'static>(neuron: &mut N){
    println!("Old membrane -> {}", neuron.get_mem().to_bits());

    let new_mem = embed_error(neuron.get_mem(), Flip);
    neuron.set_mem(new_mem);

    println!("New membrane -> {}", neuron.get_mem());

}

pub fn extra_weights_fault(weight: &mut f64){
    println!("Old Weight -> {}", weight);

    let new_weight = embed_error(*weight, Stuck0);

    *weight = new_weight;
    println!("New Weight -> {}", weight);
}