use std::slice::IterMut;
use std::sync::{Arc, Mutex};
use crate::snn::layer::Layer;
use crate::snn::neuron::Neuron;
use crate::snn::Evento;
use crate::snn::processor::Processor;

/**
    struttura rappresentante la rete neurale spiking
    - N: tipo generico che rappresenta un Neurone
    - SNN_INPUT_DIM: dimensione dell'input della rete, i.e. numero di neuroni nel primo layer
    - SNN_OUTPUT_DIM: dimensione dell'output della rete, i.e. numero di neuroni nell'ultimo layer
*/
pub struct SNN<N: Neuron + Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM: usize>{
    layers: Vec<Arc<Mutex<Layer<N>>>>
}

impl <N:Neuron + Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM: usize>
    SNN<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM> {
    pub fn new(layers: Vec<Arc<Mutex<Layer<N>>>>) -> Self {
        Self {
            layers
        }
    }

    pub fn layers(&self) -> &Vec<Layer<N>> {
        &self.layers
    }

    /**
        Processa gli inpulsi in ingresso alla rete
        - 'input': matrice di 0/1 rappresentante gli inpulsi in ingresso da processare;
            l'i-esima riga rappresenta l'ingresso della rete nell'i-esimo istante
        - 'SPIKES_DURATION': durata del treno in impulsi
        Ex:
            snn.process(&[[0,1,1], [1,0,1]])  /* 3 neuroni in ingresso, riceventi 2 impulsi ciascuno */
    */
    pub fn process<const SPIKES_DURATION: usize>(&mut self, input_spikes: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION])
                                                 -> [[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION] {
        /* trasforma in Eventi */
        let input_events = SNN::<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>::to_events(input_spikes);

        let processor = Processor {};
        let output_events = processor.process_events(self, input_events);

        /* decode output into array shape */
        let output_spikes:[[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION]
            = SNN::<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>::from_events(output_events);

        output_spikes
    }

    /** controlla e trasforma la matrice di impusli in un vettore di Eventi
        ogni riga della matrice corrisponde ad un Evento
    */
    fn to_events<const SPIKES_DURATION: usize>(spikes_matrix: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION])
                                                   -> Vec<Evento> {

        let mut eventi = Vec::<Evento>::new();
        for ts in 0..SPIKES_DURATION {
            for n in 0..SNN_INPUT_DIM {
                if spikes_matrix[ts][n] != 0 && spikes_matrix[ts][n] != 1 {
                    panic!("Error: input spike must be 0 or 1 ");
                }
            }
            let ts_spikes=spikes_matrix[ts].to_vec();
            let evento_ts=Evento::new(ts as u64, ts_spikes);
            eventi.push(evento_ts);
        }

        eventi
    }

    /**
        trasforma un vettore di Eventi in una matrice di impulsi, una riga per ogni evento
     */
    fn from_events<const SPIKES_DURATION: usize>(eventi: Vec<Evento>)
                                                   -> [[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION] {

        let mut raw_matrix = [[0u8; SNN_OUTPUT_DIM]; SPIKES_DURATION];

        if eventi.len() != SPIKES_DURATION {
            panic!("Error: number of spike events differs from the duration");
        }
        for evento in eventi {
            if evento.spikes.len() != SNN_OUTPUT_DIM{
                panic!("Error: spikes in the event should equal the output dimension")
            }
            for (n_index, spike) in evento.spikes.into_iter().enumerate() {
                raw_matrix[evento.ts as usize][n_index] = spike;
            }
        }

        raw_matrix
    }

}

impl<'a, N: Neuron+Clone+'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM : usize > IntoIterator for &'a mut SNN<N, SNN_INPUT_DIM, SNN_OUTPUT_DIM>{
    type Item = &'a mut Arc<Mutex<Layer<N>>>;
    type IntoIter = IterMut<'a, Arc<Mutex<Layer<N>>>>;

    fn into_iter(self) -> Self::IntoIter {
        self.layers.iter_mut()
    }
}