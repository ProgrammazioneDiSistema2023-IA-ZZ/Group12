use crate::models::lifneuron::LIFNeuron;
use crate::snn::snn_builder::SnnBuilder;
use std::fs::File;
use std::io::{Write, Result};
mod models;
mod snn;



fn main(){
    let mut components =Vec::<i32>::new();
    let mut error_index = -1;
    let mut n_faults = 0;
    print_menu(&mut components,&mut  error_index, &mut n_faults);
    print_configuration(&components, error_index, n_faults);


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
    let mut input = [[0,1,1], [0,0,1], [1,1,1]];
    let snn_result=snn.process(&input);
    println!("{:?}", snn_result);

    //let first_params = snn.get_params();
    //println!("{:?}", first_params);

    // let mut line = String::new();
    // std::io::stdin().read_line(&mut line).unwrap();
    // //args.name = stringa in input
    // println!("STRINGA: {}", line);

    write_configuration_to_file("output.txt", &components, error_index, n_faults, &input, &snn_result).expect("Impossible to create file!");

    println!("Params Created!")

}
fn print_menu(components: &mut Vec<i32>, error_index: &mut i32, n_faults:&mut  i32){
    print_components_menu(components);
    if components.len() != 0 {
        print_error_menu(error_index);
        print_n_fault_menu(n_faults);
    }
}
fn print_components_menu(componets: &mut Vec<i32>){
    println!("#######################################################");
    println!("\n        Spiking Neural Networks e Resilienza\n");
    println!("#######################################################");
    println!("#                                                     #");
    println!("#        Components:                                  #");
    println!("#         0 => Threshold                              #");
    println!("#         1 => Membrane                               #");
    println!("#         2 => Extra Weights                          #");
    println!("#         3 => Intra Weights                          #");
    println!("#                                                     #");
    println!("#######################################################");
    println!("Insert digit to select component to verify! - (-1 to end components selection)");
    loop {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        if trimmed_input == "-1" && componets.len() == 0{
            println!("No components selected, running without components error");
            break;
        }

        if trimmed_input == "-1" {
            println!("Components Selection Ended!");
            break;
        }

        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                if number<0 || number>3{
                    println!("Invalid component digit, try another one!");
                }else if componets.contains(&number){
                    println!("Components already inserted!, try another one!");
                }else{
                    componets.push(number);
                }

            },
            Err(_) => {
                println!("Failed to convert digit, insert another value");
            }
        }
    }

}
fn print_error_menu(error_index: &mut i32){
    println!("#######################################################");
    println!("#                                                     #");
    println!("#       Error Type:                                   #");
    println!("#         0 => Stuck-at-0                             #");
    println!("#         1 => Stuck-at-1                             #");
    println!("#         2 => Flip-bit                               #");
    println!("#                                                     #");
    println!("#######################################################");
    println!("Insert digit to select an error!");
    loop {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                if number<0 || number>2{
                    println!("Invalid error digit, try another one!");
                }else{
                    *error_index = number;
                    break;
                }
            },
            Err(_) => {
                println!("Failed to convert digit, insert another value");
            }
        }
    }

}
fn print_n_fault_menu(n_fault: &mut i32){

    println!("Insert number to select the number of faults!");
    loop {
        let mut input = String::new();
        println!("> ");
        std::io::stdin().read_line(&mut input)
            .expect("Failed to read line");

        let trimmed_input = input.trim();

        match trimmed_input.parse::<i32>() {
            Ok(number) => {
                if number<0{
                    println!("Invalid number, try another one!");
                }else{
                    *n_fault = number;
                    break;
                }
            },
            Err(_) => {
                println!("Failed to convert digit, insert another value");
            }
        }
    }

}
fn print_configuration(components: &Vec<i32>, error_index:  i32, n_faults: i32){
    let mut components_string = String::from("                                 #");
    for i in components{
        match i {
            0 => components_string += "\n#             -Threshold                              #",
            1 => components_string += "\n#             -Membrane                               #",
            2 => components_string += "\n#             -Extra Weights                          #",
            3 => components_string += "\n#             -Intra Weights                          #",
            _ => components_string += "\n#             -None                                   #",
        }
    }
    let mut error_type = String::from("                                 #");
    match error_index{
        0=> error_type += "\n#             Stuck-At-0                              #",
        1=> error_type += "\n#             Stuck-At-1                              #",
        2=> error_type += "\n#             Flip-bit                                #",
        _ => error_type += "\n#            None                                     #",
    }
    println!("#######################################################");
    println!("#                                                     #");
    println!("#       Configuration:                                #");
    println!("#                                                     #");
    println!("#         Components:{}                               ", components_string);
    println!("#                                                     #");
    println!("#         Error Type:{}                               ", error_type);
    println!("#                                                     #");
    println!("#         Number of Faults:                           #");
    println!("#             {}                                      #", n_faults);
    println!("#                                                     #");
    println!("#######################################################");
}
fn write_configuration_to_file(filename: &str, components: &Vec<i32>, error_index: i32, n_faults: i32, input: &[[u8; 3]; 3], output_r: &[[u8; 2]; 3]) -> Result<()> {
    let mut components_string = String::from("                                 #");
    for i in components {
        match i {
            0 => components_string += "\n#             -Threshold                              #",
            1 => components_string += "\n#             -Membrane                               #",
            2 => components_string += "\n#             -Extra Weights                          #",
            3 => components_string += "\n#             -Intra Weights                          #",
            _ => components_string += "\n#             -None                                   #",
        }
    }
    let mut error_type = String::from("                                 #");
    match error_index {
        0 => error_type += "\n#             Stuck-At-0                              #",
        1 => error_type += "\n#             Stuck-At-1                              #",
        2 => error_type += "\n#             Flip-bit                                #",
        _ => error_type += "\n#            None                                     #",
    }

    let mut file = File::create(filename)?;
    writeln!(file, "#######################################################")?;
    writeln!(file," \n        Spiking Neural Networks e Resilienza\n")?;
    writeln!(file, "#######################################################")?;
    writeln!(file, "#                                                     #")?;
    writeln!(file, "#       Configuration:                                #")?;
    writeln!(file, "#                                                     #")?;
    writeln!(file, "#         Components:{}                               ", components_string)?;
    writeln!(file, "#                                                     #")?;
    writeln!(file, "#         Error Type:{}                               ", error_type)?;
    writeln!(file, "#                                                     #")?;
    writeln!(file, "#         Number of Faults:                           #")?;
    writeln!(file, "#             {}                                      #", n_faults)?;
    writeln!(file, "#                                                     #")?;
    writeln!(file, "#######################################################")?;

    writeln!(file, "#          Input                                      #")?;
    writeln!(file, "#          {:?}                                       ",input)?;

    writeln!(file, "#          Output                                     #")?;
    writeln!(file, "#          {:?}                                       ",output_r)?;

    writeln!(file, "\n#######################################################")?;




    Ok(())
}