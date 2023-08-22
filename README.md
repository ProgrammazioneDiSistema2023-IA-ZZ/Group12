# Group12
# Test di resilienza su una Spiking Neural Network
- [Descrizione](#descrizione)
- [Membri del Gruppo](#membri-del-gruppo)
- [Dipendenze](#dipendenze)
- [Struttura del Repository](#struttura-del-repository)
- [Organizzazione](#organizzazione)
- [Strutture Principali](#strutture-principali)
- [Metodi Principali](#metodi-principali)
- 
## Descrizione
## Membri del Gruppo
- Andrea Sillano sxxxxxx
- Lara Moresco sxxxxxx
- Davide Palatroni s314819

## Dipendenze
- `Rust` (versione 1.56.1)
- `Cargo` (versione 1.56.0)
- `rand` (versione 0.8.5)

## Struttura del Repository
- `src/` contiene il codice sorgente  della libreria
  + `models/` contiene le specifiche implementazioni dei modelli (in questo caso solo `Lif Neuron`)
  + `snn/` contiene l'implementazione generica della SNN 
  + 
## Organizzazione
La libreria è organizzata come segue:
- ### Builder
  Il modulo `Builder` permette di creare la struttura della rete con i rispettivi layerscx
- ### Rete
## Strutture Principali
La libreria provvede le seguenti strutture:

- `LIFNeuron` rappresenta il neurone per il modello `Leaky Integrate and Fire` e può venir utilizzato per creare un `Layer` di neuroni:

```rust
pub struct LIFNeuron{
        /* campi costanti */
        v_th: f64, /* potenziale di soglia */
        v_rest: f64, /* potenziale di riposo */
        v_reset: f64, /* potenziale di reset */
        tau: f64,
        d_t: f64, /* intervallo di tempo tra due istanti successivi */
        /*campi mutabili*/
        v_mem: f64, /* potenziale di membrana */
        t_s: u64 /* ultimo istante di tempo in cui ha ricevuto almeno un impulso */
}
```

- `Layer` rappresenta uno strato di neuroni, viene utilizzato per costruire i layer della `SNN`.

```rust
pub struct Layer<N: Neuron+Clone+'static>{
        neurons: Vec<N>, /* neuroni del layer */
        weights: Vec<Vec<f64>>, /* pesi tra i neuroni di questo layer con quelli del layer precedente */
        intra_weights: Vec<Vec<f64>>, /* pesi tra i neuroni dello stesso layer */
        prev_output: Vec<u8>, /* impulsi di output al precedente istante */
}
```

- `Evento` rappresenta un evento di uno strato di neuroni che si attiva ad un determinato istante di tempo. Incapsula gli impulsi che passano attraverso la rete.
```rust
pub struct Evento{
        ts: u64, /* istante di tempo in cui viene generato l'output */
        spikes: Vec<u8>, /* vettore che contiene tutti gli output */
}
```

- `SNN` rappresenta la `Spike Neural Network` composta da un vettore di `Layer`

```rust
pub struct SNN<N: Neuron + Clone + 'static, const SNN_INPUT_DIM: usize, const SNN_OUTPUT_DIM: usize> {
  layers: Vec<Arc<Mutex<Layer<N>>>>
}
```

- `Processor` è l'oggetto che ha il compito di gestire i thread dei layer e processare gli impulsi di input 
```rust
pub struct Processor { }
```

- `SNNBuilder` rappresenta il builder per una `SNN`
```rust
pub struct SnnBuilder<N: Neuron+Clone+Debug+'static>{
        params: SnnParams<N>
}

pub struct SnnParams<N: Neuron+Clone+Debug+'static>{
        neurons: Vec<Vec<N>>, /* neuroni per ciascun layer */
        extra_weights: Vec<Vec<Vec<f64>>>, /* pesi (positivi) tra i vari layer */
        intra_weights: Vec<Vec<Vec<f64>>>, /* pesi (negativi) all'interno dello stesso layer */
}
```

- `InfoTable` rappresenta la struttura necessaria per generare il report sulla resilienza. Immagazzina tutte le informazioni riguardo agli errori generati e come influiscono sulla `SNN` in questione.
```rust
pub struct InfoTable{
  layers: Vec<usize>,
  neurons: Vec<usize>,
  components: Vec<usize>,
  bits: Vec<usize>,
  error_type: Vec<usize>,
  accuracy: Vec<f64>,
  counter: i32
}
```

## Metodi Principali
