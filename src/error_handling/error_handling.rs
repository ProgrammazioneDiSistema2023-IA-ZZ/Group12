use crate::error_handling::error_handling::ErrorType::{Flip, Stuck0, Stuck1};
use crate::snn::neuron::Neuron;

#[derive(Copy, Clone)]
pub enum ErrorType{
    Stuck0,
    Stuck1,
    Flip,
    None
}

const ERROR_TABLE: [ErrorType; 4] = [ErrorType::Stuck0, ErrorType::Stuck1, ErrorType::Flip, ErrorType::None];
/// Ritorna il valore della variabile con il bit modificato
/// # Argomenti
/// * `variable` - variabile con il bit da modificare
/// * `error` - ErrorType che indica il tipo di errore sul bit
/// * `position` - posizione del bit da modificare nella variabile
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
/**
Setta a 0 il bit in posizione *position* di *value*
 */
fn unset_bit(value:u64, position:u8)->u64{
    /* maschera del tipo 1110111 */
    let bit_mask=!(1u64<<position);
    value & bit_mask
}
/**
Inverte il bit in posizione *position* di *value*
 */
fn flip_bit(value:u64, position:u8)->u64{
    /* maschera del tipo 0001000 */
    let bit_mask=1u64<<position;
    value ^ bit_mask
}
/// Inserisce un nuovo errore sul potenziale di membrana. Che sia transitorio o no, il valore deve essere settato una sola volta
/// # Argomenti
/// * `neuron` - neurone il cui potenziale di soglia contiene l'errore
/// * `error_type` - tipo di errore da inserire
/// * `position` - posizione del bit affetto da errore
pub fn threshold_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, position:u8){
    /* calcolo del nuovo valore */
    let new_threshold = embed_error(neuron.get_th(), ERROR_TABLE[error_type as usize], position);

    neuron.set_th(new_threshold);

}
/// Inserisce un nuovo errore sul potenziale di membrana.
/// Se è un errore transitorio, il valore può essere settato una volta solamente,
/// altrimenti il neurone deve ricordarsi la presenza dell'errore stuck
/// # Argomenti
/// * `neuron` - neurone il cui potenziale di soglia contiene l'errore
/// * `error_type` - tipo di errore da inserire
/// * `position` - posizione del bit affetto da errore
pub fn membrane_fault<N: Neuron+Clone+'static>(neuron: &mut N, error_type: i32, position: u8){
    match error_type {
        // stuck-at-X
        0|1=>{neuron.set_membrane_error(error_type as u8,position);},
        // transient
        2=>{let new_mem = embed_error(neuron.get_mem(), ERROR_TABLE[error_type as usize], position);
            neuron.set_mem(new_mem);},
        _=>{}
    }
}
/// Inserisce un nuovo errore sul un peso. Che sia transitorio o no, il valore deve essere settato una sola volta
/// # Argomenti
/// * `weight` - reference al peso affetto da errore
/// * `error_type` - tipo di errore da inserire
/// * `position` - posizione del bit affetto da errore
pub fn weight_fault(weight: &mut f64, error_type: i32, position: u8){

    let new_weight = embed_error(*weight, ERROR_TABLE[error_type as usize], position);

    *weight = new_weight;
}
pub fn adder_fault(sum: &mut f64, error_type: i32, position: u8){
    let new_sum = embed_error(*sum, ERROR_TABLE[error_type as usize], position);
    *sum = new_sum;
}
pub fn adder_fault_input(input: &mut f64, error_type: i32, position: u8){
    let new_input = embed_error(*input, ERROR_TABLE[error_type as usize], position);
    *input = new_input;
}
pub fn mult_fault(sum: &mut f64, error_type: i32, position: u8){
    let new_sum = embed_error(*sum, ERROR_TABLE[error_type as usize], position);
    *sum = new_sum;
}
pub fn mult_fault_input(input: &mut f64, error_type: i32, position: u8){
    let new_input = embed_error(*input, ERROR_TABLE[error_type as usize], position);
    *input = new_input;
}