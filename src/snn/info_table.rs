use std::fs::File;
use std::io::Write;
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
#[derive(Debug)]
pub struct InfoTable {
    layers: Vec<usize>,
    neurons: Vec<usize>,
    components: Vec<usize>,
    bits: Vec<usize>,
    error_type: Vec<usize>,
    accuracy: Vec<f64>
}

impl InfoTable {
    pub fn new()->Self{
        Self{
            layers: vec![],
            neurons: vec![],
            components: vec![],
            bits: vec![],
            error_type: vec![],
            accuracy: vec![],
        }
    }
    pub fn add_layer(&mut self, layer_index: usize){
        self.layers.push(layer_index);
    }
    pub fn add_neuron(&mut self, neuron_index: usize){
        self.neurons.push(neuron_index);
    }
    pub fn add_component(&mut self, component_index: usize){
        self.components.push(component_index);
    }
    pub fn add_bit(&mut self, bit_index: usize){
        self.bits.push(bit_index);
    }
    pub fn add_error_type(&mut self, error_type: usize){
        self.error_type.push(error_type);
    }
    pub fn add_output(&mut self, acc: f64){
        self.accuracy.push(acc);
    }
    pub fn print_table(&mut self, ){
        let len =  self.layers.len() ;
        let mut table = vec![];
        for n in  0..len{
            table.push(vec![self.layers[n].cell().justify(Justify::Right),
                            self.neurons[n].cell().justify(Justify::Right),
                            from_index_to_str_component(self.components[n]).cell().justify(Justify::Right),
                            self.bits[n].cell().justify(Justify::Right),
                            from_index_to_str_error(self.error_type[n]).cell().justify(Justify::Right),
                            (self.accuracy[n].to_string()+"%").cell().justify(Justify::Right)
            ])
        }
        let table_complete = table.table().title(vec!["Layer".cell().bold(true), "Neuron".cell().bold(true), "Component".cell().bold(true), "Bit".cell().bold(true),"Error".cell().bold(true), "Impact On Accuracy".cell().bold(true)]);
        let table_display = table_complete.display().unwrap();
        print!("{}", table_display);
    }
    pub fn print_table_file(&mut self, file: &mut File){
        let len =  self.layers.len() ;
        writeln!(file,"+-------+--------+-------------+-----+------------+--------------------+");
        writeln!(file,"| Layer | Neuron |  Component  | Bit |    Error   | Impact On Accuracy |");
        writeln!(file,"+-------+--------+-------------+-----+------------+--------------------+");
        for n in  0..len{
            writeln!(file," {}        {}       {}   {}     {}              {} ",
                     self.layers[n],
                     self.neurons[n],
                     from_index_to_str_component(self.components[n]),
                     self.bits[n],
                     from_index_to_str_error(self.error_type[n]),
                     format!("{}%", self.accuracy[n]));
            writeln!(file,"+-------+--------+-------------+-----+------------+--------------------+");

        }

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