use crate::snn::neuron::Neuron;
// struttura per salvare un errore stuck-at-X
#[derive(Clone, Debug)]
struct ErrorBit{
    error_type:u8, //0,1
    position:u8,
}
impl ErrorBit {
    pub fn new(error_type: u8, position: u8) -> Self {
        Self { error_type, position }
    }
}
#[derive(Debug)]
pub struct LIFNeuron{
    /* campi costanti */
    v_th: f64, /* potenziale di soglia */
    v_rest: f64, /* potenziale di riposo */
    v_reset: f64, /* potenziale di reset */
    tau: f64,
    d_t: f64, /* intervallo di tempo tra due istanti successivi */
    /*campi mutabili*/
    v_mem: f64, /* potenziale di membrana */
    t_s: u64, /* ultimo istante di tempo in cui ha ricevuto almeno un impulso */
    membrane_error:Option<ErrorBit>
}

impl LIFNeuron {

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
    fn check_error(&mut self){
        if self.membrane_error.is_none(){return;}
        let error=self.membrane_error.as_ref().unwrap();
        let mask=1u64<<error.position;
        println!("Old Membrane -> {}", self.v_mem);
        match error.error_type {
            0 => { self.v_mem = f64::from_bits(self.v_mem.to_bits() & !mask) },
            1=> { self.v_mem = f64::from_bits(self.v_mem.to_bits() | mask) },
            _=>{}
        }
        println!("New Membrane -> {}", self.v_mem);
    }
}

impl Neuron for LIFNeuron{
    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64) -> u8 {
        let weight_sum = intra_weight+extra_weight;

        let exponent = -(((t-self.t_s)as f64)*self.d_t)/self.tau;
        self.check_error();
        self.v_mem = self.v_rest + (self.v_mem-self.v_rest)*exponent.exp() + weight_sum;

        self.t_s = t;
        self.check_error();
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
