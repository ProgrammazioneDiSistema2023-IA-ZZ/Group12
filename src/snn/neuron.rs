pub trait Neuron: Send{
/// Funzione per calcolare il nuovo potenziale di membrana del neurone;
/// Ritorna un segnale binario 0/1
/// # Argomenti
/// * `t` - Instante di tempo corrente
/// * `intra_weight` - Somma pesata dei segnali provenienti dal layer stesso
/// * `extra_weight` - Somma pesata dei segnali provenienti dal layer precedente
    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64)->u8;
/// Funzione per resettare i parametri del neurone a quelli iniziali
    fn init_neuron(&mut self);
/// Funzione per per settare un errore stuck-at-X sul potenziale di membrana
/// # Argomenti
/// * `error_type` - valore a cui il bit è bloccato *(0/1)*
/// * `position` - posizione del bit bloccato
    fn set_membrane_error(&mut self, error_type:u8, position:u8);
/// Ritorna il valore del potenziale di soglia
    fn get_th(&self) -> f64;
/// Setta il valore del potenziale di soglia
    fn set_th(&mut self, new_th: f64);
/// Ritorna il valore del potenziale di membrana
    fn get_mem(&self) -> f64;
/// Setta il valore del potenziale di membrana
    fn set_mem(&mut self, new_mem: f64);

}