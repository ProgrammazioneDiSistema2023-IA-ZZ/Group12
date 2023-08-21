use rand::Rng;
use crate::snn;
use crate::snn::error_handling::ErrorType::{Flip, Stuck0, Stuck1};
use crate::snn::info_table::InfoTable;
use crate::snn::neuron::Neuron;

#[derive(Clone)]
pub enum ErrorType{
    Stuck0,
    Stuck1,
    Flip,
    None
}

const ERROR_TABLE: [ErrorType; 4] = [ErrorType::Stuck0, ErrorType::Stuck1, ErrorType::Flip, ErrorType::None];

fn embed_error(variable:f64, error:ErrorType, info_table: &mut InfoTable)->f64{
    let mut bit_value:u64=variable.to_bits();
    let mut rng = rand::thread_rng();
    let position: u32 = rng.gen_range(0..64);
    info_table.add_bit(position as usize);
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

pub fn threshold_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, info_table: &mut InfoTable){
    println!("Old Threshold -> {}", neuron.get_th());
    let new_threshold = embed_error(neuron.get_th(), ERROR_TABLE[error_type as usize].clone(), info_table);

    neuron.set_th(new_threshold);
    println!("New Threshold -> {}", neuron.get_th());

}

pub fn membrane_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, info_table: &mut InfoTable){
    println!("Old membrane -> {}", neuron.get_mem());

    let new_mem = embed_error(neuron.get_mem(), ERROR_TABLE[error_type as usize].clone(), info_table);
    neuron.set_mem(new_mem);

    println!("New membrane -> {}", neuron.get_mem());

}

pub fn extra_weights_fault(weight: &mut f64, error_type: i32, info_table: &mut InfoTable){
    println!("Old Weight -> {}", weight);

    let new_weight = embed_error(*weight, ERROR_TABLE[error_type as usize].clone(), info_table);

    *weight = new_weight;
    println!("New Weight -> {}", weight);
}