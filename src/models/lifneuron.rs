use crate::snn::neuron::Neuron;
#[derive(Debug)]
pub struct LIFNeuron{
    v_th: f64,
    v_rest: f64,
    v_reset: f64,
    tau: f64,
    d_t: f64,

    v_mem: f64,
    t_s: u64
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
        }
    }

    pub fn v_th(&self) -> f64 {
        self.v_th
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
}

impl Neuron for LIFNeuron{
    fn update_v_mem(&mut self, t: u64, intra_weight: f64, extra_weight: f64) -> u8 {
        let weight_sum = intra_weight+extra_weight;

        let exponent = -(((t-self.t_s)as f64)*self.d_t)/self.tau;

        self.v_mem = self.v_rest + (self.v_mem-self.v_rest)*exponent.exp() + weight_sum;

        self.t_s = t;

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
        }
    }
}
