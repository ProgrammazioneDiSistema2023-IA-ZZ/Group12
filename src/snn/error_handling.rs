use rand::Rng;
use crate::snn;
use crate::snn::error_handling::ErrorType::{Flip, Stuck0, Stuck1};
use crate::snn::info_table::InfoTable;
use crate::snn::neuron::Neuron;

#[derive(Copy, Clone)]
pub enum ErrorType{
    Stuck0,
    Stuck1,
    Flip,
    None
}

const ERROR_TABLE: [ErrorType; 4] = [ErrorType::Stuck0, ErrorType::Stuck1, ErrorType::Flip, ErrorType::None];

fn embed_error(variable:f64, error:ErrorType, position: u8)->f64{
    let mut bit_value:u64=variable.to_bits();
    bit_value=match error {
        ErrorType::None => bit_value,
        Stuck0 => unset_bit(bit_value, position),
        Stuck1 => set_bit(bit_value, position),
        Flip => flip_bit(bit_value, position),
    };

    f64::from_bits(bit_value)
}

/**
Setta a 1 il bit in posizione *position* di *value*
 */
fn set_bit(value:u64, position:u8)->u64{
    /* maschera del tipo 0001000 */
    let bit_mask=1u64<<position;
    value | bit_mask
}
fn unset_bit(value:u64, position:u8)->u64{
    /* maschera del tipo 1110111 */
    let bit_mask=!(1u64<<position);
    value & bit_mask
}
fn flip_bit(value:u64, position:u8)->u64{
    /* maschera del tipo 0001000 */
    let bit_mask=1u64<<position;
    value ^ bit_mask
}

pub fn threshold_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, position:u8){

    let new_threshold = embed_error(neuron.get_th(), ERROR_TABLE[error_type as usize], position);

    neuron.set_th(new_threshold);

}

pub fn membrane_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, position: u8){
    match error_type {
        0|1=>{neuron.set_membrane_error(error_type as u8,position);},
        2=>{let new_mem = embed_error(neuron.get_mem(), ERROR_TABLE[error_type as usize], position);
            neuron.set_mem(new_mem);},
        _=>{}
    }
}

pub fn weight_fault(weight: &mut f64, error_type: i32, position: u8){

    let new_weight = embed_error(*weight, ERROR_TABLE[error_type as usize], position);

    *weight = new_weight;
}