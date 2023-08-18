use crate::models::lifneuron::LIFNeuron;
use crate::snn::snn_builder::SnnBuilder;
mod models;
mod snn;



fn main(){
    let mut snn = SnnBuilder::new().add_layer().add_weight([
        [0.1, 0.2],
        [0.3, 0.4],
        [0.5, 0.6]
    ]).add_neurons([
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25, -0.3],
        [-0.10, 0.0, -0.3],
        [-0.1, -0.3,  0.0]
    ])
    .add_layer()
        .add_weight([
            [0.1, 0.2, 0.3],
            [0.4, 0.5, 0.6]
        ]).add_neurons([
        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
        LIFNeuron::new(0.3, 0.05, 0.1, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],
        [-0.10, 0.0]
    ]).clone().build::<3,2>();
    let snn_result=snn.process(&[[0,1,1], [0,0,1], [1,1,1]]);
    println!("{:?}", snn_result);

    //let first_params = snn.get_params();
    //println!("{:?}", first_params);

    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    //args.name = stringa in input
    println!("STRINGA: {}", line);

    println!("Params Created!")

}