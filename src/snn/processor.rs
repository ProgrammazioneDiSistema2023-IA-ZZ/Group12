use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use std::thread;
use std::thread::JoinHandle;
use crate::snn::components::{Adder, Multiplier};
use crate::snn::Evento;
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
/// Struttura utilizzata per processare l'input della rete neurale
#[derive(Debug)]
pub struct Processor { }

impl Processor {

/// Struttura per processare l'input della rete. Ritorna l'output della rete.
/// Questa funzione crea un thread per ogni layer della rete in modo da parallelizzare i calcoli;
/// ogni layer è in comunicazione con il layer precedente e quello successivo
/// con *channels* per scambire Eventi rappresentanti gli impulsi
/// # Argomenti
/// * `snn` - rete neurale che deve processare gli impulsi
/// * `spikes` - vettore degli Eventi in input; ogni Evento un vettore di impulsi in ingresso ad un determinato istante
    pub fn process_events<'a, N: Neuron+Clone+'static, S: IntoIterator<Item=&'a mut Arc<Mutex<Layer<N>>>>>
        (self, snn: S, spikes: Vec<Evento>, adder: Adder , mult:  Multiplier) -> Vec<Evento>{
        /* Creiamo la pool di tutti i thread */
         let mut threads  = Vec::<JoinHandle<()>>::new();

        /*  Creiamo il channel per comunicare con il primo layer.
            net_input_tx serve per mandare l'input della rete (spikes)
            layer_rc sarà il receiver del primo layer;
        */
        let (net_input_tx, mut layer_rc) = channel::<Evento>();

        /* Creiamo channels tra ogni layer */
        for layer_ref in snn {

            let layer_ref = layer_ref.clone();
            /* layer_tx manderà l'output di questo layer al next_layer_rc, ovvero il receiver del prossimo layer */
            let (layer_tx, next_layer_rc) = channel::<Evento>();

            /* Creiamo il thread che processa il layer */
            let thread  = thread::spawn(move|| {
                /* Blocchiamo il layer in considerazione */
                let mut layer = layer_ref.lock().unwrap();
                /* Eseguiamo il compito del layer */
                layer.process(adder,mult,layer_rc, layer_tx);
            });
            /* Inseriamo il thread all'interno del vettore con tutti i thread creati */
            threads.push(thread);
            /* il layer_rc del layer successivo è aggiornato
                con il receiver del channel aperto da questo layer*/
            layer_rc = next_layer_rc;

        }
        /* layer_rc è rimasto il Receiver del channel dell'ultimo layer*/
        let net_output_rc = layer_rc;

        for evento in spikes {

            let instant = evento.ts;

            /* Mandiamo l'input al primo layer; la precedente creazione
                dei channels farà in modo che gli impulsi vengano propagati nella rete */
            net_input_tx.send(evento)
                .expect(&format!("ERROR: sending spikes event at t={}", instant));
            // generiamo un messaggio di errore particolare in caso di errore **/
        }
        /* droppando il Sender al primo layer, faremo terminare in cascata tutti i thread */
        drop(net_input_tx);

        /* Vettore di eventi per l'output di uscita */
        let mut spikes_output = Vec::<Evento>::new();

        /* Aspettiamo che arrivino tutti gli eventi di output dall'ultimo layer*/
        while let Ok(spike_event) = net_output_rc.recv() {
            spikes_output.push(spike_event);
        }

        /* Aspettiamo che riceva tutti i thread */
        for thread in threads {
            thread.join().unwrap();
        }

        spikes_output
    }


}