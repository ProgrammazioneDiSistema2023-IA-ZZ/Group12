use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread::JoinHandle;
use crate::snn::Evento;
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;

pub struct Processor { }
impl Processor {
    pub fn process_events<'a, N: Neuron+Clone+'static, S: IntoIterator<Item=&'a mut Arc<Mutex<Layer<N>>>>>(self, snn: S, spikes: Vec<Evento>) -> Vec<Evento>{
        /** Creiamo la pool di tutti i thread **/
         let mut threads  = Vec::<JoinHandle<()>>::new();

        /** Creiamo il channel a cui dare il primo layer **/
        let (net_input_tx, mut layer_rc) = channel::<Evento>();

        /** Creiamo il TX di input e il receiver per ciascun layer **/

        for layer_ref in snn {

            /** Creiamo il channel per il prossimo layer **/
            let (layer_input, next_layer_rc) = channel::<Evento>();


        }
        let net_output_rc = layer_rc;

        let mut spikes_output = Vec::<Evento>::new();

        while let Ok(spike_event) = net_output_rc.recv() {
            
        }

        /** Aspettiamo che riceva tutti i thread **/
        for thread in threads {
            thread.join().unwrap();
        }

        spikes_output
    }


}