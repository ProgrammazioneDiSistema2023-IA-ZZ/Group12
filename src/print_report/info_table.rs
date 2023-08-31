use std::io::Error;
use std::fs::File;
use std::io::Write;
use cli_table::{format::Justify, Cell, Style, Table};
use strip_ansi_escapes::strip;

/// Struttura per salvare le informazioni di tutti gli errori inseriti nella rete e, per ogni inserimento,
/// l'accuratezza dell'output della rete con l'errore
#[derive(Debug)]
pub struct InfoTable{
    layers: Vec<usize>,
    neurons: Vec<usize>,
    components: Vec<usize>,
    bits: Vec<usize>,
    error_type: Vec<usize>,
    accuracy: Vec<f64>,
    counter: i32,
    error_input: Vec<(i32,i32)>
}

impl InfoTable {
    pub fn new() -> Self {
        Self {
            layers: vec![],
            neurons: vec![],
            components: vec![],
            bits: vec![],
            error_type: vec![],
            accuracy: vec![],
            counter: 0,
            error_input: vec![],
        }
    }
    /// Aggiunge l'indice del layer in cui viene iniettato l'errore
    pub fn add_layer(&mut self, layer_index: usize) {
        self.layers.push(layer_index);
    }
    /// Aggiunge l'indice del neurone in cui viene iniettato l'errore
    pub fn add_neuron(&mut self, neuron_index: usize) {
        self.neurons.push(neuron_index);
    }
    /// Aggiunge il componente in cui viene iniettato l'errore
    pub fn add_component(&mut self, component_index: usize) {
        self.components.push(component_index);
    }
    /// Aggiunge l'indice del bit in cui viene iniettato l'errore
    pub fn add_bit(&mut self, bit_index: usize) {
        self.bits.push(bit_index);
    }
    /// Aggiunge il tipo di errore che viene iniettato
    pub fn add_error_type(&mut self, error_type: usize) {
        self.error_type.push(error_type);
    }
    /// Aggiunge accuratezza dell'output
    pub fn add_output(&mut self, acc: f64) {
        if acc != 0.0 {
            self.counter += 1;
        }
        self.accuracy.push(acc);
    }
    pub fn add_error_inputs(&mut self, input1: i32, input2: i32) {
        if input1 == 3 && input2 ==  3 {
            self.error_input.push((0,0));
        }else if input1 != 3 && input2 ==  3 {
            self.error_input.push((1,0));
        }else if input1 == 3 && input2 !=  3{
            self.error_input.push((0,1));
        }else if input1 != 3 && input2 !=  3{
            self.error_input.push((1,1));
        }

    }
    pub fn print_no_error<const SNN_OUTPUT_DIM: usize, const SPIKES_DURATION: usize, const SNN_INPUT_DIM: usize>(&self, file: &mut File,snn_result_0_error: &[[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION],  snn_input: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION])->Result<(),Error>{
        println!("#######################################################");
        println!("#                  SNN WITHOUT ERROR                  #");
        println!("#######################################################");
        println!("INPUT: {:?}                                   ", snn_input);
        println!("#######################################################");
        println!("OUPUT: {:?}                                   ", snn_result_0_error);
        println!("#######################################################");

        writeln!(file,"#######################################################")?;
        writeln!(file,"#                  SNN WITHOUT ERROR                  #")?;
        writeln!(file,"#######################################################")?;
        writeln!(file,"INPUT: {:?}                                   ", snn_input)?;
        writeln!(file,"#######################################################")?;
        writeln!(file,"OUPUT: {:?}                                   ", snn_result_0_error)?;
        writeln!(file,"#######################################################")?;
        Ok(())

    }
    /// Stampa su file una tabella con tutte le informazioni sugli errori
    pub fn print_table(&mut self, file: &mut File) -> Result<(), Error> {
        let multiplier = 10_f64.powi(2);
        let mut best_indecies: Vec<usize>= vec![];
        let max_impact = self.accuracy.clone().into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let impacted_inferences = 100 as f64 * self.accuracy.clone().into_iter().filter(|&x| x > 0.0).count() as f64 / self.layers.len() as f64;
        let non_zero_values: Vec<f64> = self.accuracy.clone().into_iter().filter(|&x| x != 0.0).collect();

        let mut avarage_impact = 0.0;
        if !non_zero_values.is_empty() {
            let sum: f64 = non_zero_values.iter().sum();
            avarage_impact = sum / non_zero_values.len() as f64;
        }
        if max_impact != 0.0{
            best_indecies = self.accuracy.iter().enumerate().filter(|&(_, &x)| x == max_impact).map(|(index, _)| index).collect();
        }
        let len = self.layers.len();
        let mut table = vec![];
        for n in 0..len {
            let truncated_imp = (self.accuracy[n]* multiplier).floor() / multiplier;
            let mut layer= self.layers[n].cell().justify(Justify::Right);
            let mut neuron = self.neurons[n].cell().justify(Justify::Right);
            if  self.components[n] == 4 || self.components[n] == 5 ||  self.components[n] == 6 || self.components[n] == 7 {
               layer = "/".cell().justify(Justify::Right);
                neuron="/".cell().justify(Justify::Right);
            }
            let mut input = from_index_to_str_component(self.components[n]).cell().justify(Justify::Left);
            if self.components[n] == 5 || self.components[n] == 7{
                input =(from_index_to_str_component(self.components[n]).to_string() +" - ("+ &*self.error_input[n].0.to_string() +","+ &*self.error_input[n].1.to_string() +")").cell().justify(Justify::Left);

            }

            table.push(vec![layer,
                            neuron,
                            input,
                            self.bits[n].cell().justify(Justify::Right),
                            from_index_to_str_error(self.error_type[n] ).cell().justify(Justify::Left),
                            (truncated_imp.to_string() + "%").cell().justify(Justify::Right)
            ])
        }
        let table_complete = table.table().title(vec!["Layer".cell().bold(true), "Neuron".cell().bold(true), "Component".cell().bold(true), "Bit".cell().bold(true), "Error".cell().bold(true), "Impact On Accuracy".cell().bold(true)]);
        let table_display = table_complete.display().unwrap();


        print!("{}", table_display);
        //Stampa una tabella con un riassunto delle informazioni più importanti


        let stripped_bytes = strip(table_complete.display().unwrap().to_string());
        let stripped_table = String::from_utf8_lossy(&stripped_bytes);
        file.write_all(stripped_table.as_bytes()).expect("TEST");
        if max_impact != 0.0{
            print_max_impact_info(file,self.layers.clone(), self.neurons.clone(), self.components.clone(), self.bits.clone(), self.error_type.clone(), self.accuracy.clone(), self.error_input.clone(), best_indecies.clone()).expect("Unable to write");
        }
        print_summary_table(file, self.counter, impacted_inferences, max_impact, avarage_impact).expect("Error");

        Ok(())
    }
}
fn print_max_impact_info(file: &mut File, layers: Vec<usize>, neurons: Vec<usize>, components: Vec<usize>, bits:Vec<usize>, error_type: Vec<usize>, accuracy: Vec<f64>, error_input: Vec<(i32, i32)>, best_indecies: Vec<usize>)->Result<(),Error>{
    println!("\n######################################################################################");
    println!("#                                   MAX IMPACT INFO                                  #");
    println!("######################################################################################");
    let multiplier = 10_f64.powi(2);
    let mut table = vec![];

    for n in best_indecies {
        let truncated_imp = (accuracy[n]* multiplier).floor() / multiplier;
        let mut layer= layers[n].cell().justify(Justify::Right);
        let mut neuron = neurons[n].cell().justify(Justify::Right);
        if  components[n] == 4 || components[n] == 5 ||  components[n] == 6 || components[n] == 7 {
            layer = "/".cell().justify(Justify::Right);
            neuron="/".cell().justify(Justify::Right);
        }
        let mut input = from_index_to_str_component(components[n]).cell().justify(Justify::Left);
        if components[n] == 5 || components[n] == 7{
            input =(from_index_to_str_component(components[n]).to_string() +" - ("+ &*error_input[n].0.to_string() +","+ &*error_input[n].1.to_string() +")").cell().justify(Justify::Left);

        }

        table.push(vec![layer,
                        neuron,
                        input,
                        bits[n].cell().justify(Justify::Right),
                        from_index_to_str_error(error_type[n] ).cell().justify(Justify::Left),
                        (truncated_imp.to_string() + "%").cell().justify(Justify::Right)
        ])
    }


    let table_complete = table.table().title(vec!["Layer".cell().bold(true), "Neuron".cell().bold(true), "Component".cell().bold(true), "Bit".cell().bold(true), "Error".cell().bold(true), "Impact On Accuracy".cell().bold(true)]);
    let table_display = table_complete.display().unwrap();
    print!("{}", table_display);

    writeln!(file,"\n######################################################################################")?;
    writeln!(file,"#                                   MAX IMPACT INFO                                  #")?;
    writeln!(file,"######################################################################################")?;
    let stripped_bytes = strip(table_complete.display().unwrap().to_string());
    let stripped_table = String::from_utf8_lossy(&stripped_bytes);
    file.write_all(stripped_table.as_bytes()).expect("TEST");
    Ok(())


}
fn print_summary_table(file: &mut File, tot_inf: i32, impacted_inf: f64, max_impact: f64, avarge_impact: f64)->Result<(), Error>{
    println!("\n######################################################################################");
    println!("#                                       SUMMARY                                      #");
    println!("######################################################################################");
    let multiplier = 10_f64.powi(2);
    let truncated_max = (max_impact * multiplier).floor() / multiplier;
    let truncated_avg = (avarge_impact * multiplier).floor() / multiplier;
    let mut table = vec![];
    table.push(vec![tot_inf.cell().justify(Justify::Right)
                            ,(impacted_inf.to_string()+"%").cell().justify(Justify::Right),
                    (truncated_max.to_string() +"%").cell().justify(Justify::Right),
                    (truncated_avg.to_string() +"%").cell().justify(Justify::Right)]);
    let table_complete = table.table().title(vec!["Total Affected Inferences".cell().bold(true), "Total Affected Inferences %".cell().bold(true), "Max Impact On Accuracy".cell().bold(true), "Average Impact On Accuracy".cell().bold(true)]);
    println!("{}", table_complete.display().unwrap());
    writeln!(file,"\n######################################################################################")?;
    writeln!(file,"#                                       SUMMARY                                      #")?;
    writeln!(file,"######################################################################################")?;
    let stripped_bytes = strip(table_complete.display().unwrap().to_string());
    let stripped_table = String::from_utf8_lossy(&stripped_bytes);
    file.write_all(stripped_table.as_bytes()).expect("TEST");
    Ok(())

}

fn from_index_to_str_error(index: usize) -> &'static str {
    match index{
        0=>"Stack-At-0",
        1=>"Stack-At-1",
        2=>"Flip-Bit",
        _ => "None"
    }
}
fn from_index_to_str_component(index: usize) -> &'static str {
    match index{
        0=>"Threshold",
        1=>"Membrane",
        2=>"Extra Weight",
        3=>"Intra Weight",
        4=>"Adder Output",
        5=>"Adder Input",
        6=>"Multiplier Output",
        7=>"Multiplier Input",
        _ => "None"
    }
}
