use std::io::Error;
use std::fs::File;
use std::io::Write;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use strip_ansi_escapes::strip;
#[derive(Debug)]
pub struct InfoTable {
    layers: Vec<usize>,
    neurons: Vec<usize>,
    components: Vec<usize>,
    bits: Vec<usize>,
    error_type: Vec<usize>,
    accuracy: Vec<f64>,
    counter: i32
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
        }
    }
    pub fn add_layer(&mut self, layer_index: usize) {
        self.layers.push(layer_index);
    }
    pub fn add_neuron(&mut self, neuron_index: usize) {
        self.neurons.push(neuron_index);
    }
    pub fn add_component(&mut self, component_index: usize) {
        self.components.push(component_index);
    }
    pub fn add_bit(&mut self, bit_index: usize) {
        self.bits.push(bit_index);
    }
    pub fn add_error_type(&mut self, error_type: usize) {
        self.error_type.push(error_type);
    }
    pub fn add_output(&mut self, acc: f64) {
        if acc != 0.0 {
            self.counter += 1;
        }
        self.accuracy.push(acc);
    }
    pub fn print_table(&mut self, file: &mut File) -> Result<(), Error> {
        let max_impact = self.accuracy.clone().into_iter().max_by(|a, b| a.partial_cmp(b).unwrap()).unwrap();
        let impacted_inferences = 100 as f64 * self.accuracy.clone().into_iter().filter(|&x| x > 0.0).count() as f64 / self.layers.len() as f64;

        let len = self.layers.len();
        let mut table = vec![];
        for n in 0..len {
            table.push(vec![self.layers[n].cell().justify(Justify::Right),
                            self.neurons[n].cell().justify(Justify::Right),
                            from_index_to_str_component(self.components[n]).cell().justify(Justify::Right),
                            self.bits[n].cell().justify(Justify::Right),
                            from_index_to_str_error(self.error_type[n]).cell().justify(Justify::Right),
                            (self.accuracy[n].to_string() + "%").cell().justify(Justify::Right)
            ])
        }
        let table_complete = table.table().title(vec!["Layer".cell().bold(true), "Neuron".cell().bold(true), "Component".cell().bold(true), "Bit".cell().bold(true), "Error".cell().bold(true), "Impact On Accuracy".cell().bold(true)]);
        let table_display = table_complete.display().unwrap();
        print!("{}", table_display);
        println!("#######################################################");
        println!("# Number of Affected inferences: {}                   ", self.counter);
        println!("# Max impact on accuracy: {}%                         ", max_impact);
        println!("# Inferences impacted : {}%                           ", impacted_inferences);
        let stripped_bytes = strip(table_complete.display().unwrap().to_string());
        let stripped_table = String::from_utf8_lossy(&stripped_bytes);
        file.write_all(stripped_table.as_bytes()).expect("TEST");
        writeln!(file,"#######################################################")?;
        writeln!(file,"# Number of Affected inferences: {}                   ", self.counter)?;
        writeln!(file,"# Max impact on accuracy: {}%                         ", max_impact)?;
        writeln!(file,"# Inferences impacted : {}%                           ", impacted_inferences)?;
        Ok(())
    }
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
        _ => "None"
    }
}