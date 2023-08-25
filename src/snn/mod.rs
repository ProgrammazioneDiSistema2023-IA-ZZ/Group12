pub mod neuron; /* pubblico */
pub mod snn_builder; /*privato */
    mod layer;
    mod processor;
    mod snn;
mod error_handling;
pub(crate) mod info_table;
pub mod menu_handler;


/// Struttura che rappresenta gli impulsi scambiati tra i layer della rete
/// in un determinato istante
pub struct Evento {
    ts: u64, /* istante di tempo in cui viene generato l'output */
    spikes: Vec<u8>, /* vettore che contiene tutti gli output */
}

impl Evento {
    pub fn new(ts: u64, spikes: Vec<u8>) -> Self{
        Self{ ts, spikes }
    }
}