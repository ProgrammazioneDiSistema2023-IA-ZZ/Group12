use std::fs::File;
use std::io::Write;
use std::io::Error;

pub fn print_menu(components: &mut Vec<i32>, error_index: &mut i32, n_faults:&mut  i32){
    print_components_menu(components);
    if components.len() != 0 {
        print_error_menu(error_index);
        print_n_fault_menu(n_faults);
    }
}
pub fn print_components_menu(componets: &mut Vec<i32>){
    println!("#######################################################");
    println!("\n        Spiking Neural Networks e Resilienza\n");
    println!("#######################################################");
    println!("#                                                     #");
    println!("#        Components:                                  #");
    println!("#         0 => Threshold                              #");
    println!("#         1 => Membrane                               #");
    println!("#         2 => Extra Weights                          #");
    println!("#         3 => Intra Weights                          #");
    println!("#         4 => Adder Output                           #");
    println!("#         5 => Adder Input                            #");
    println!("#         6 => Multiplier Output                      #");
    println!("#         7 => Multiplier Input                       #");
    println!("#         8 => All Components                         #");
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
                if number<0 || number>8{
                    println!("Invalid component digit, try another one!");
                }else if componets.contains(&number){
                    println!("Components already inserted!, try another one!");
                }else if number == 8{
                    componets.clear();
                    for i in 0..8{
                            componets.push(i);
                        }
                    println!("All components selected");

                    break;
                }else{
                    componets.push(number);
                    if componets.len() == 8 {
                        break;
                    }
                }

            },
            Err(_) => {
                println!("Failed to convert digit, insert another value");
            }
        }
    }

}
pub fn print_error_menu(error_index: &mut i32){
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
pub fn print_n_fault_menu(n_fault: &mut i32){

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
pub fn print_configuration(components: &Vec<i32>, error_index:  i32, n_faults: i32){
    let mut components_string = String::from("                                 #");
    for i in components{
        match i {
            0 => components_string += "\n#             -Threshold                              #",
            1 => components_string += "\n#             -Membrane                               #",
            2 => components_string += "\n#             -Extra Weights                          #",
            3 => components_string += "\n#             -Intra Weights                          #",
            4 => components_string += "\n#             -Adder Output                           #",
            5 => components_string += "\n#             -Adder Input                            #",
            6 => components_string += "\n#             -Multiplier Output                      #",
            7 => components_string += "\n#             -Multiplier Input                       #",
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
pub fn write_configuration_to_file(file: &mut File, components: &Vec<i32>, error_index: i32, n_faults: i32) -> Result<(), Error> {
    let mut components_string = String::from("                                 #");
    for i in components {
        match i {
            0 => components_string += "\n#             -Threshold                              #",
            1 => components_string += "\n#             -Membrane                               #",
            2 => components_string += "\n#             -Extra Weights                          #",
            3 => components_string += "\n#             -Intra Weights                          #",
            4 => components_string += "\n#             -Adder Output                           #",
            5 => components_string += "\n#             -Adder Input                            #",
            6 => components_string += "\n#             -Multiplier Output                      #",
            7 => components_string += "\n#             -Multiplier Input                       #",
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


    Ok(())
}