use crate::error_handling::error_handling;
use crate::error_handling::error_handling::ErrorType;
/// Struttura rappresentante il componente elaborativo Sommatore in un circuito.
/// Può essere affetto da errore sugli ingressi o sull'uscita
#[derive(Clone,Debug, Copy)]
pub struct Adder{
    /// tipo di errore sull'uscita; ha valore `3` se non c'è errore
    error:i32,
    /// posizione del bit affetto da errore
    position: u8,
    /// opzionale tipo di errore sugli ingressi;
    /// l'errore può essere presente su entrambi gli ingressi o uno solamente.
    /// e.g.
    /// * stuck-at-0 solo sul primo ingresso: `(0,3)`,
    /// * stuck-at-0 solo sul secondo ingresso: `(3,0)`,
    /// * stuck-at-0 su entrambi gli ingressi: `(0,0)`,
    input: Option<(i32,i32)>
}
/// Struttura rappresentante il componente elaborativo Moltiplicatore in un circuito.
/// Può essere affetto da errore sugli ingressi o sull'uscita
#[derive(Clone,Debug, Copy)]
pub struct Multiplier{
    /// tipo di errore sull'uscita; ha valore `3` se non c'è errore
    error:i32,
    /// posizione del bit affetto da errore
    position: u8,
    /// opzionale tipo di errore sugli ingressi;
    /// l'errore può essere presente su entrambi gli ingressi o uno solamente.
    /// e.g.
    /// * stuck-at-0 solo sul primo ingresso: `(0,3)`,
    /// * stuck-at-0 solo sul secondo ingresso: `(3,0)`,
    /// * stuck-at-0 su entrambi gli ingressi: `(0,0)`,
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
    /// Funzione equivalente a **`input1 + input2`**, ma aggiunge eventuali errori
    /// sui valori di ingresso o quello d'uscita
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
                error_handling::adder_fault_input(&mut inp2,err2, self.position);
                inp1+inp2
            }
        }

    }
    /// Funzione equivalente a **`input1 - input2`**, ma aggiunge eventuali errori
    /// sui valori di ingresso o quello d'uscita
    pub fn sub(&self, input1:f64, input2:f64)->f64{
        match  self.input{
            None =>{
                /* Nessun errore sull'ingresso, possibile errore sull'uscita */
                let mut  sub = input1-input2;
                error_handling::adder_fault(&mut sub, self.error, self.position);
                sub
            },
            Some((err,err2))=>{
                let mut inp1 = input1;
                let mut inp2 = input2;
                error_handling::adder_fault_input(&mut inp1,err, self.position);
                error_handling::adder_fault_input(&mut inp2,err2, self.position);
                inp1-inp2
            }
        }
    }
    /// Setta i parametri di errore su un bit del valore in uscita
    pub fn set_params(&mut self, err: i32,pos: u8){
        self.position = pos;
        self.error = err;
        /* Se si è aggiunto un errore in uscita, forziamo l'assenza di errori in ingresso*/
        self.input=None;
    }
    /// Setta i parametri di errore su un bit di uno o entrambi i valori in ingresso
    pub fn set_params_input(&mut self, pos: u8, err_input1: i32,err_input2: i32){
        self.position = pos;
        /* Se si è aggiunto un errore in ingresso, forziamo l'assenza di errori in uscita*/
        self.error = 3;
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
    /// Funzione equivalente a **`input1 * input2`**, ma aggiunge eventuali errori
    /// sui valori di ingresso o quello d'uscita
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
                error_handling::mult_fault_input(&mut inp2,err2, self.position);
                inp1*inp2
            }
        }

    }
    /// Funzione equivalente a **`input1 / input2`**, ma aggiunge eventuali errori
    /// sui valori di ingresso o quello d'uscita
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
                error_handling::mult_fault_input(&mut inp2,err2, self.position);
                inp1/inp2
            }
        }
    }
    /// Setta i parametri di errore su un bit del valore in uscita
    pub fn set_params(&mut self, err: i32,pos: u8){
        self.position = pos;
        self.error = err;
        /* Forziamo l'assenza di errori in input*/
        self.input=None;
    }
    /// Setta i parametri di errore su un bit di uno o entrambi i valori in ingresso
    pub fn set_params_input(&mut self, pos: u8, err_input1: i32,err_input2: i32){
        self.position = pos;
        self.error = 3;
        self.input = Some((err_input1, err_input2));
    }
}








