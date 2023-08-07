    mod snn_builder; /* privato */
pub mod neuron; /* pubblico */
    mod layer;


/**
    Oggetto rappresentante l'output generato da un single layer
*/

pub struct Evento {

    ts: u64, /* istante di tempo in cui l'evento viene generato */
    spikes: Vec<u8>, /* vettore degli output generati da quell'evento */
    
}

impl Evento {
    pub fn new(ts: u64, spikes: Vec<u8>) -> Self {
        Self { ts, spikes }
    }
}