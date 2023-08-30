use crate::models::lifneuron::LIFNeuron;
use crate::snn::snn_builder::SnnBuilder;
use crate::snn::info_table::InfoTable;
use crate::snn::menu_handler;
use std::fs::File;
mod models;
mod snn;


fn main(){
    let mut components =Vec::<i32>::new();
    let mut error_index = -1;
    let mut n_faults = 0;
    let mut table = InfoTable::new();
    let mut file = File::create("output.txt").expect("Unable to create file");

    menu_handler::print_menu(&mut components,&mut  error_index, &mut n_faults);
    menu_handler::print_configuration(&components, error_index, n_faults);

    let mut binding = SnnBuilder::new();
    let builder = binding.add_layer().add_weight([
        [0.1, 0.2],
        [0.3, 0.4],
        [0.5, 0.6]
    ]).add_neurons([
                        LIFNeuron::new(0.03, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.05, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.09, 0.05, 0.1, 1.0, 1.0),
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
        LIFNeuron::new(0.07, 0.04, 0.4, 1.0, 1.0),
        LIFNeuron::new(0.3, 0.01, 0.4, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],
        [-0.10, 0.0]
    ]).add_layer().add_weight([
        [0.1, 0.2],
        [0.3, 0.4],
        [0.5, 0.6]
    ]).add_neurons([
        LIFNeuron::new(0.03, 0.01, 0.1, 1.0, 1.0),
        LIFNeuron::new(0.05, 0.03, 0.2, 1.0, 1.0),
        LIFNeuron::new(0.09, 0.06, 0.4, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25, -0.3],
        [-0.10, 0.0, -0.3],
        [-0.1, -0.3,  0.0]
    ]).add_layer()
        .add_weight([
            [0.1, 0.2, 0.3],
            [0.4, 0.5, 0.6]
        ]).add_neurons([
        LIFNeuron::new(0.07, 0.01, 0.2, 1.0, 1.0),
        LIFNeuron::new(0.03, 0.08, 0.3, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],
        [-0.10, 0.0]
    ]);

    let input = [[0,1,1], [0,0,1], [1,1,1], [1,0,0], [0,0,1], [0,1,0]];
    /* SNN WITHOUT ANY ERROR */
    let mut snn_0_error = builder.clone().build::<3,2>(&Vec::new(), -1, &mut table);
    let snn_result_0_error= snn_0_error.process(&input);
    /* SNN WITH ERRORS */
    for _ in 0..n_faults {
        let mut snn = builder.clone().build::<3,2>(&components, error_index, &mut table);
        let snn_result= snn.process(&input);
        let acc = calculate_accuracy(&snn_result_0_error, &snn_result);
        table.add_output((1.0-acc)*100.0);
        println!("{:?}", snn_result);
    }

    menu_handler::write_configuration_to_file(&mut file, &components, error_index, n_faults).expect("Impossible to create file!");
    table.print_table(&mut file).expect("Unable To write Statics");

}
fn calculate_accuracy<const SNN_OUTPUT_DIM: usize, const SPIKES_DURATION: usize>(v1: &[[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION], v2: &[[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION]) -> f64 {
    let total_elements = v1.iter().map(|row| row.len()).sum::<usize>();
    let matching_elements = v1.iter().zip(v2.iter())
        .map(|(row1, row2)| row1.iter().zip(row2.iter()).filter(|&(elem1, elem2)| elem1 == elem2).count())
        .sum::<usize>();

    let accuracy = matching_elements as f64 / total_elements as f64;
    accuracy
}

