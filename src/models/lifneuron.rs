use crate::snn::neuron::Neuron;
use crate::error_handling::components::{Adder, Multiplier};
/// Struttura che rappresenta un errore stuck-at-X su un determinato bit
#[derive(Clone, Debug)]
struct ErrorBit{
    error_type:u8, //0/1
    position:u8,
}
impl ErrorBit {
/// Ritorna un nuovo **ErrorBit*
/// # Argomenti
/// * `error_type` - valore a cui il bit è bloccato *(0/1)*
/// * `position` - posizione del bit bloccato
    pub fn new(error_type: u8, position: u8) -> Self {
        Self { error_type, position }
    }
}
/// Struttura che rappresenta un neurone di tipo *Leaky Integrate and Fire* (**LIF**)
#[derive(Debug)]
pub struct LIFNeuron{
    /* campi costanti */
    /// potenziale di soglia
    v_th: f64,
    /// potenziale di riposo
    v_rest: f64,
    ///potenziale di reset
    v_reset: f64,
    /// costante di tempo *tau=C·R*
    tau: f64,
    /// intervallo di tempo tra due istanti successivi
    d_t: f64,
    /*campi mutabili*/
    /// potenziale di membrana
    v_mem: f64,
    /// ultimo istante di tempo in cui si è ricevuto un impulso
    t_s: u64,
    /// *eventuale* errore su un bit del potenziale di membrana
    membrane_error:Option<ErrorBit>
}

impl LIFNeuron {
/// Ritorna un nuovo neurone di tipo LIF
/// # Argomenti
/// * `v_th` - potenziale di soglia
/// * `v_rest` - potenziale di riposo
/// * `v_reset` - potenziale di reset
/// * `tau` - constante di tempo
/// * `d_t` - intervallo di tempo tra due istanti
/// # Valori predefiniti
/// * `v_mem` - potenziale di membrana settato al potenziale di riposo
/// * `t_s` - istante di tempo iniziale settato a 0
/// * `membrane_error` - nessun errore sulla memprana (Option::None)
    pub fn new(v_th: f64, v_rest: f64, v_reset: f64, tau: f64, d_t: f64)-> Self{
        Self{
            v_th,
            v_rest,
            v_reset,
            tau,
            d_t,
            v_mem: v_rest,
            t_s: 0u64,
            membrane_error:None
        }
    }
    /*** getters ***/
    pub fn v_rest(&self) -> f64 {
        self.v_rest
    }
    pub fn v_reset(&self) -> f64 {
        self.v_reset
    }
    pub fn tau(&self) -> f64 {
        self.tau
    }
    pub fn d_t(&self) -> f64 {
        self.d_t
    }
    pub fn v_mem(&self) -> f64 {
        self.v_mem
    }
    pub fn t_s(&self) -> u64 {
        self.t_s
    }
/// Funzione per controllare la presenza di un errore stuck-at-X su un bit del
/// potenziale di membrana ed eventualmente forzare tale bit ad X
    fn check_error(&mut self){
        if self.membrane_error.is_none(){return;}
        let error=self.membrane_error.as_ref().unwrap();
        let mask=1u64<<error.position;
        match error.error_type {
            /* stuck-at-0 */
            0 => { self.v_mem = f64::from_bits(self.v_mem.to_bits() & !mask) },
            /* stuck-at-1 */
            1=> { self.v_mem = f64::from_bits(self.v_mem.to_bits() | mask) },
            _=>{}
        }
    }
}

impl Neuron for LIFNeuron{
    /* in caso di un errore stuck-at-X sul potenziale di membrana, questo errore
        deve essere forzato prima di ciascun utilizzo del potenziale *(i.e. formula e confronto)* */
    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64, adder:  Adder, mult:  Multiplier) -> u8 {
        let weight_sum = adder.add(intra_weight,extra_weight);
        //let weight_sum = intra_weight+extra_weight;
        let exponent = -mult.div(mult.mul(adder.sub(t as f64, self.t_s as f64),self.d_t as f64),self.tau as f64);
       // let exponent = -(((t-self.t_s)as f64)*self.d_t)/self.tau;
        /* controllo sull'errore su v_mem prima del suo utilizzo */
        self.check_error();
        self.v_mem = adder.add(adder.add(self.v_rest ,mult.mul(adder.sub(self.v_mem,self.v_rest),exponent.exp())),weight_sum);
       // self.v_mem = self.v_rest + (self.v_mem-self.v_rest)*exponent.exp() + weight_sum;

        self.t_s = t;
        /* controllo sull'errore su v_mem prima del suo confronto con la soglia */
        self.check_error();
        /* confronto con la soglia ed ritorno del segnale*/
        return if self.v_mem > self.v_th {
            self.v_mem = self.v_reset;
            1
        } else {
            0
        }
    }




    fn init_neuron(&mut self) {
        self.v_mem= self.v_rest;
        self.t_s = 0u64;
        self.membrane_error = None;
    }
    fn set_membrane_error(&mut self, error_type:u8, position:u8){
        self.membrane_error =Some(ErrorBit::new(error_type, position));
    }
    fn get_th(&self) -> f64 {
        self.v_th
    }

    fn set_th(&mut self, new_th: f64) {
        self.v_th = new_th;
    }

    fn get_mem(&self) -> f64 {
        self.v_mem
    }

    fn set_mem(&mut self, new_mem: f64){
        self.v_mem = new_mem;
    }
}

impl Clone for LIFNeuron{
    fn clone(&self) -> Self {
        Self{
            v_th: self.v_th,
            v_rest: self.v_rest,
            v_reset: self.v_reset,
            tau: self.tau,
            d_t: self.d_t,
            v_mem: self.v_mem,
            t_s: self.t_s,
            membrane_error:self.membrane_error.clone()
        }
    }
}
