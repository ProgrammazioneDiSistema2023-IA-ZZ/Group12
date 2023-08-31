use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
use crate::snn::snn::SNN;
use rand::Rng;
use rand::rngs::ThreadRng;
use crate::snn::error_handling;
use crate::snn::info_table::InfoTable;
use crate::snn::components::{Adder, Multiplier};

/// Enumeratore che identifica il tipo di errore da inserire nella rete
pub enum ErrorComponent{
    ThresholdError,
    IntraWeightsError,
    ExtraWeightsError,
    MembraneError,
    AdderOutputError
}
/// Struttura che contiene i parametri della rete che si sta cotruendo
#[derive(Debug, Clone)]
pub struct SnnParams<N: Neuron+ Clone+Debug+'static>{
    /// Vettori di neuroni per ciascun layer
    neurons: Vec<Vec<N>>,
    /// Vettori di pesi tra i neuroni di layer differenti;
    /// - il primo indice indica il layer
    /// - il secondo indice indica il neurone a cui arriva il peso
    /// - il terzo indice il neurone del layer precedente da cui arriva il peso
    extra_weights: Vec<Vec<Vec<f64>>>,
    /// Vettori di pesi tra i neuroni dello stesso layer
    intra_weights: Vec<Vec<Vec<f64>>>,
}
/// Struttura per creare la rete neurale aggiornando i suoi parametri
#[derive(Debug, Clone)]
pub struct SnnBuilder<N: Neuron+Clone+Debug+'static>{
    params: SnnParams<N>,
    adder: Adder,
    mult: Multiplier
}

impl <N: Neuron+ Clone+Debug> SnnBuilder<N> {

    pub fn new()->Self{
        Self {
            params: SnnParams {
                neurons: vec![],
                extra_weights: vec![],
                intra_weights: vec![],
            },
            adder: Adder::new(3,0),
            mult: Multiplier::new(3,0),
        }
    }
    pub fn get_params(&self) -> SnnParams<N> {
        self.params.clone()
    }

    /** Per ora serve solo logicamente */
    pub fn add_layer(&mut self) -> &mut SnnBuilder<N> {
        self
    }

/// Aggiunge un nuovo layer di pesi esterni e controlla che siano tutti pesi positivi
/// # Argomenti
/// * `weights` - vettori di pesi tra i neuroni dell'ultimo layer con i neuroni del layer precedente
    pub fn add_weight<const NUM_NEURONS: usize, const PREVIOUS_DIM: usize >(&mut self, weights:[[f64; PREVIOUS_DIM]; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &weights{
            for w in n_weight{
                if w < &0.0{
                    panic!("Pesi devono essere positivi!");
                }
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.extra_weights.push(new_weights);
        self
    }
/// Aggiunge un nuovo layer di neuroni
    pub fn add_neurons<const NUM_NEURONS: usize>(&mut self, neurons: [N; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        self.params.neurons.push(Vec::from(neurons));
        self
    }
/// Aggiunge un nuovo layer di pesi interni e controlla che siano tutti pesi negativi
/// # Argomenti
/// * `intra_weights` - vettori di pesi tra i neuroni
    pub fn add_intra_weights<const NUM_NEURONS: usize>(&mut self, intra_weights: [[f64; NUM_NEURONS]; NUM_NEURONS]) -> &mut SnnBuilder<N> {
        let mut new_weights = <Vec<Vec<f64>>>::new();

        for n_weight in &intra_weights{
            for w in n_weight{
                if w>&0f64{
                    panic!("Pesi nello stesso layer devono essere negativi!");
                }
            }
            new_weights.push(Vec::from(n_weight.as_slice()));
        }
        self.params.intra_weights.push(new_weights);
        self
    }
/// Funzione per la scelta casuale di un layer e di un neurone all'interno di quest'ultimo.
/// Ritorna una tupla contenente gli indici di layer e neurone
    fn choose_neuron(neurons: &Vec<Vec<N>>, rng: &mut ThreadRng) -> (usize,usize){
        let n_layers = rng.gen_range(0..neurons.len());
        let n_neuron = rng.gen_range(0..neurons[n_layers].len());

        (n_layers,n_neuron)
    }
/// Funzione che ritorna un indice casuale in un vettore di pesi
    fn weight_index(weights: &Vec<f64>, rng: &mut ThreadRng) -> usize{
        rng.gen_range(0..weights.len())
    }
/// Funzione per generare a caso la presenza di un errore su uno solo o entrambi gli ingressi di un blocco elaborativo
    fn generate_input_error(rng: &mut ThreadRng, error_type:i32)->(i32,i32){
        let index = rng.gen_range(0..3);
        match index{
            /* solo sul primo ingresso */
            0 => (error_type, 3),
            /* solo sul secondo ingresso */
            1 => (3, error_type),
            /* su entrambi */
            2 => (error_type, error_type),
            _ => (3, 3)
        }
    }


/// Funzione per gestire l'iniezione di errori all'interno della rete
/// # Argomenti
/// * `components` - lista di valori per indicare i possibili componenti in cui iniettare l'errore.
///   Possibili valori:
///     - `0` -> Potenziale di soglia
///     - `1` -> Potenziale di membrana
///     - `2` -> Pesi esterni
///     - `3` -> Pesi interni
///     - `4` -> Adder output
///     - `5` -> Adder input
///     - `6` -> Multiplier output
///     - `7` -> Multiplier input
/// * `error_type` - tipo di errore da iniettare nella rete:
///     - `0` -> Stuck-at-0
///     - `1` -> Stuck-at-1
///     - `2` -> Transient bit-flip
/// * `info_table` - struttura per salvare le informazioni di tutti gli errori inseriti
/// * `transient_error` - variabile opzionale per salvare le informazioni temporanee di un errore transitorio
    fn handle_errors(&mut self, components: &Vec<i32>, error_type: i32, info_table: &mut InfoTable, transient_error: &mut Option<(usize, usize, i32, u8,(i32,i32))>){
        let mut rng = rand::thread_rng();
        if components.len()==0 {return;}
        let component_index = rng.gen_range(0..components.len());
        /* scelta casuale di uno dei componenti */
        let component = components[component_index];
        let position: u8 = rng.gen_range(0..64);

        info_table.add_component(component as usize);
        info_table.add_error_type(error_type as usize);
        info_table.add_bit(position as usize);

        let (layer_index, neuron_index) = SnnBuilder::choose_neuron(&self.params.neurons.clone(),&mut rng);
        info_table.add_layer(layer_index);
        info_table.add_neuron(neuron_index);

        let (err_input1, err_input2) = SnnBuilder::<N>::generate_input_error(&mut rng, error_type);
        /* In base al tipo di errore e componente selezionato, si possono verificare tre casi generali:
            1- stuck-at-X su parametri costanti (i.e. soglia e pesi): Il bit deve essere settato solo all'inizio
            2- stuck-at-X su membrana: deve essere garantito X ad ogni variazione del valore (i.e. ogni volta che il neurone processa un input)
            3- transient-bit-flip su qualsiasi componente: valore settato una volta sola, ma ad un istante casuale (verrà iniettato da Snn.process())
        */

        info_table.add_error_inputs(err_input1,err_input2);
        match (component,error_type) {
            //stuck_at_X on threshold
            (0,0)|(0,1)=>{
                error_handling::threshold_fault(&mut self.params.neurons[layer_index][neuron_index], error_type, position);

            },
            //stuck-at-X on membrane
            (1,0)|(1,1)=>{
                error_handling::membrane_fault(&mut self.params.neurons[layer_index][neuron_index], error_type, position);

            },
            //stuck-at-X on extra-weights
            (2,0)|(2,1)=>{
                let w=&mut self.params.extra_weights[layer_index][neuron_index];
                let idx=SnnBuilder::<N>::weight_index(w, &mut rng);
                error_handling::weight_fault(&mut w[idx],error_type,position);

            },
            //stuck-at-X on intra-weights
            (3,0)|(3,1)=>{
                let w=&mut self.params.intra_weights[layer_index][neuron_index];
                let idx=SnnBuilder::<N>::weight_index(w, &mut rng);
                error_handling::weight_fault(&mut w[idx],error_type,position);

            },
            //transient error
            (0,2)|(1,2)|(2,2)|(3,2)|(4,2)|(5,2)|(6,2)|(7,2)=> {
                *transient_error = Some((layer_index, neuron_index, component, position, (err_input1, err_input2)));
            },
            //stuck_at_X on Adder output
            (4,0)|(4,1)=>{self.adder.set_params(error_type, position);

            }
            //stuck_at_X on Adder input(s)
            (5,0)|(5,1)=>{
                self.adder.set_params_input(position,err_input1,err_input2);

            },
            //stuck_at_X on Multiplier output
            (6,0)|(6,1)=>{
                self.mult.set_params(error_type,position);

            },
            //stuck_at_X on Multiplier input(s)
            (7,0)|(7,1)=>{
                self.mult.set_params_input(position,err_input1,err_input2);

            }
            (_,_)=>{}
        }
    }
/// Funzione che crea la rete SNN dai parametri di costruzione
/// # Argomenti
/// * `components` - lista di valori per indicare i possibili componenti in cui iniettare l'errore.
///   Possibili valori:
///     - `0` -> Potenziale di soglia
///     - `1` -> Potenziale di membrana
///     - `2` -> Pesi esterni
///     - `3` -> Pesi interni
///     - `4` -> Adder output
///     - `5` -> Adder input
///     - `6` -> Multiplier output
///     - `7` -> Multiplier input
/// * `error_type` - tipo di errore da iniettare nella rete:
///     - `0` -> Stuck-at-0
///     - `1` -> Stuck-at-1
///     - `2` -> Transient bit-flip
/// * `info_table` - struttura per salvare le informazioni di tutti gli errori inseriti
    pub fn build<const INPUT_DIM: usize, const OUTPUT_DIM:usize>(&mut self, components: &Vec<i32>, error_type: i32, info_table: &mut InfoTable) -> SNN<N, { INPUT_DIM }, { OUTPUT_DIM }>{
        if self.params.extra_weights.len() != self.params.neurons.len() || self.params.intra_weights.len() != self.params.neurons.len(){
            panic!("Wrong number bewteen layers!")
        }

        let mut layers: Vec<Arc<Mutex<Layer<N>>>> = Vec::new();
        let mut transient: Option<(usize, usize,i32, u8, (i32,i32))>=None;
        self.handle_errors(components,error_type,info_table,&mut transient);

        let mut n_iter = self.params.neurons.clone().into_iter();
        let mut extra_iter = self.params.extra_weights.clone().into_iter();
        let mut intra_iter = self.params.intra_weights.clone().into_iter();

        while let Some(layer) = n_iter.next() {
            let new_extra_iter = extra_iter.next().unwrap();
            let new_intra_iter = intra_iter.next().unwrap();

            /* creazione di un nuovo layer */
            let new_layer = Layer::new(layer, new_extra_iter, new_intra_iter);
            layers.push(Arc::new(Mutex::new(new_layer)));
        }
        SNN::<N, {INPUT_DIM }, { OUTPUT_DIM }>::new(layers, transient, self.adder.clone(), self.mult.clone())
    }


}