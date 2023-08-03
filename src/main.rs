use crate::models::lifneuron::LIFNeuron;
use crate::snn::snn_builder::SnnBuilder;
mod models;
mod snn;


fn main(){
    let mut snn = SnnBuilder::new();
    snn.add_layer().add_weight([
        [0.1, 0.2],     /* weigths from input layer to the 1st neuron */
        [0.3, 0.4],     /* weigths from input layer to the 2nd neuron */
        [0.5, 0.6]      /* weigths from input layer to the 3rd neuron */
    ]).add_neurons([
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],       /* weigths from the same layer to the 1st neuron */
        [-0.10, 0.0]        /* weigths from the same layer to the 2nd neuron */
    ]);

    println!("Params Created!")

}