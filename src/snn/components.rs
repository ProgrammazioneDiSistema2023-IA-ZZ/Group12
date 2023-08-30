use crate::snn::error_handling;
use crate::snn::error_handling::ErrorType;
#[derive(Clone,Debug, Copy)]
pub struct Adder{
    error:i32,
    position: u8,
    input: Option<(i32,i32)>
}
#[derive(Clone,Debug, Copy)]
pub struct Multiplier{
    error:i32,
    position: u8,
    input: Option<(i32,i32)>
}
impl Adder{
    pub fn new(err: i32,pos: u8)->Self{
        Self{
            error: err,
            position: pos,
            input: None
        }
    }
    pub fn add(&self, input1:f64, input2:f64)->f64{
        match  self.input{
            None =>{
                let mut  sum = input1+input2;
                error_handling::adder_fault(&mut sum, self.error, self.position);
                sum
            },
            Some((err,err2))=>{
                let mut inp1 = input1;
                let mut inp2 = input2;
                error_handling::adder_fault_input(&mut inp1,err, self.position);
                error_handling::adder_fault_input(&mut inp1,err2, self.position);
                inp1+inp2
            }
        }

    }
    pub fn sub(&self, input1:f64, input2:f64)->f64{
        match  self.input{
            None =>{
                let mut  sub = input1-input2;
                error_handling::adder_fault(&mut sub, self.error, self.position);
                sub
            },
            Some((err,err2))=>{
                let mut inp1 = input1;
                let mut inp2 = input2;
                error_handling::adder_fault_input(&mut inp1,err, self.position);
                error_handling::adder_fault_input(&mut inp1,err2, self.position);
                inp1-inp2
            }
        }
    }
    pub fn set_params(&mut self, err: i32,pos: u8){
        self.position = pos;
        self.error = err;
    }
    pub fn set_params_input(&mut self, err: i32,pos: u8, err_input1: i32,err_input2: i32){
        self.position = pos;
        self.error = err;
        self.input = Some((err_input1, err_input2));
    }
}
impl Multiplier{
    pub fn new(err: i32,pos: u8)->Self{
        Self{
            error: err,
            position: pos,
            input: None
        }
    }
    pub fn mul(&self, input1:f64, input2:f64)->f64{
        match  self.input{
            None =>{
                let mut  mul = input1*input2;
                error_handling::mult_fault(&mut mul, self.error, self.position);
                mul
            },
            Some((err,err2))=>{
                let mut inp1 = input1;
                let mut inp2 = input2;
                error_handling::mult_fault_input(&mut inp1,err, self.position);
                error_handling::mult_fault_input(&mut inp1,err2, self.position);
                inp1*inp2
            }
        }

    }
    pub fn div(&self, input1:f64, input2:f64)->f64{
        match  self.input{
            None =>{
                let mut  div = input1/input2;
                error_handling::mult_fault(&mut div, self.error, self.position);
                div
            },
            Some((err,err2))=>{
                let mut inp1 = input1;
                let mut inp2 = input2;
                error_handling::mult_fault_input(&mut inp1,err, self.position);
                error_handling::mult_fault_input(&mut inp1,err2, self.position);
                inp1/inp2
            }
        }
    }
    pub fn set_params(&mut self, err: i32,pos: u8){
        self.position = pos;
        self.error = err;
    }
    pub fn set_params_input(&mut self, err: i32,pos: u8, err_input1: i32,err_input2: i32){
        self.position = pos;
        self.error = err;
        self.input = Some((err_input1, err_input2));
    }
}








