use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use crate::snn::Evento;
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;

#[derive(Debug)]
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
            let (layer_tx, next_layer_rc) = channel::<Evento>();

            /** Creiamo effettivamente il thread **/
            let thread  = thread::spawn(move || {
                /** Prendiamo il layer in considerazione **/
                let mut layer = layer_ref.lock().unwrap();
                /** Eseguiamo il compito del layer **/
                layer.process(layer_rc, layer_tx)
            });

            threads.push(thread); /* Inseriamo il thread all'interno del vettore con tutti i thread creati */
            layer_rc = next_layer_rc; /* Aggiorniamo il layer_rc, per passarlo al prossimo layer */

        }
        let net_output_rc = layer_rc;

        for evento in spikes {
            if evento.spikes.iter().all(|spike|{*spike == 0u8}){
                continue;
            }


            let instant = evento.ts;

            /** Mandiamo l'input agli altri thread **/
            net_input_tx.send(evento)
                .expect(&format!("ERROR: sending spikes event at t={}", instant)); /** generiamo un messaggio di errore particolare in caso di errore **/
        }

        drop(net_input_tx); /** droppiamo net_input_tx, così da far terminare tutti i thread **/

        /** Vettore di eventi per l'output di uscita **/
        let mut spikes_output = Vec::<Evento>::new();

        /** Aspettiamo che arrivi l'ultimo output per inserirlo nel vettore di uscita **/
        while let Ok(spike_event) = net_output_rc.recv() {
            spikes_output.push(spike_event);
        }

        /** Aspettiamo che riceva tutti i thread **/
        for thread in threads {
            thread.join().unwrap();
        }

        spikes_output
    }


}