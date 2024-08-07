# Group12
# Test di resilienza su una Spiking Neural Network
- [Descrizione](#descrizione)
- [Membri del Gruppo](#membri-del-gruppo)
- [Dipendenze](#dipendenze)
- [Struttura del Repository](#struttura-del-repository)
- [Organizzazione](#organizzazione)
- [Strutture Principali](#strutture-principali)
- [Metodi Principali](#metodi-principali)
- [Esempio di Utilizzo](#esempio-di-utilizzo)

## Descrizione
Questo repository ha lo scopo di simulare una serie di possibili errori in una `Spiking Neural Network` e di studiarne la resilienza. La serie di errori che 
vengono simulati e i vari componenti coinvolti sono indicati in seguito. 
Il repository prevede una possibile implementazione di una `Spiking Neural Network` ma non prevede il supporto per la fase di training della rete, solo quella
di esecuzione.

## Membri del Gruppo
- Andrea Sillano s314771
- Lara Moresco s320153
- Davide Palatroni s314819

## Dipendenze
- `Rust` (versione 1.56.1)
- `Cargo` (versione 1.56.0)
- `rand` (versione 0.8.5)
- `cli-table` (versione 0.4)
- `strip-ansi-escapes` (versione 0.2.0)

## Struttura del Repository
- `src/` contiene il codice sorgente  della libreria
    + `error_handling/` contiene tutta la simulazione dell'errore sui componenti
    + `models/` contiene le specifiche implementazioni dei modelli (in questo caso solo `LIFNeuron`)
    + `print_report/` contiene tutte le informazioni relative alla stampa e al calcolo delle statistiche
    + `snn/` contiene l'implementazione generica della SNN
## Organizzazione
La libreria è organizzata come segue:
- ### Builder
Il modulo `Builder` permette di creare la struttura della rete con i rispettivi layers, neuroni per ciascun layer, i corrispettivi pesi tra neuroni dello stesso layer e tra layer diversi. In particolare il modulo
`SnnBuilder` permette di allocare *staticamente* una `Spike Neural Network` prendendo per ciascun layer un vettore statico di neuroni, uno di pesi e un altro di pesi tra i vari layer. La libreria può controllare
la correttezza della struttura della rete a *compile time*, ma questo implica che tutte le strutture di rete sono allocate nello **Stack** (**Non adatta a reti molto grandi**).

- ### Rete
Il modulo `Network` permette di eseguire la rete dato un determinato input. In particolare `Snn` viene creato da `SnnBuilder` e permette di processare un dato input attraverso il metodo `process()`.
Come `SnnBuilder`, `Snn` riceve l'input come un vettore statico di inpulsi e produce come output un vettore dinamico di inpulsi. La correttezza dell'input può essere controllata a *compile-time*. 
- ### Gestione dell'errore
Il modulo `Error Handling` permette di simulare uno tra gli errori richiesti sulla rete. All'interno di esso vi è stata inserita un *enum* che specifica il tipo di errore da simulare. Le possibilità offerte dalla libreria sono:
  - `ErrorType::Stuck0`: simula lo `Stuck-At-0`, ovvero il bit rimane fisso a **0**, anche se richiesto il contrario;
  - `ErrorType::Stuck1`: simula lo `Stuck-At-1`, ovvero il bit rimane fisso a **1**, anche se richiesto il contrario;
  - `ErrorType::Flip`: simula il `transient bit flip`, ovvero il valore del bit viene invertito;
  - `ErrorType::None`: simula il corretto funzionamento della rete, senza nessun errore;
Tra tali errori, lo `Stuck-At-X` viene applicato forzando il bit al valore definito (**0 oppure 1**) per tutta la durata dell'inferenza mentre il `transient bit flip` ha validità solo in uno specifico istante di tempo ed 
eventuali nuove scritture non subiscono tale errore.

Le strutture su cui è possbile studiarne il comportamento sono:
- `Potenziale di Soglia`
- `Potenziale di Membrana`;
- `Pesi`, che possono essere `Intra-Weights`, ovvero i pesi tra due neuroni appartenenti allo stesso layer, oppure `Extra-Weights`, ovvero pesi tra due neuroni appartenenti a due layer diversi
- `Blocchi Elaborativi`, che a loro volta possono essere:
  - `Adder`, che simula il sommatore;
  - `Multiplier`, che simula il moltiplicatore e il divisore;
l'errore simulato sui componenti hardware può riguardare l'input (solo uno oppure entrambi) oppure l'output;

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
- `Adder` simula il componente che opera le operazioni di addizione e sottrazione in un sistema di elaborazione
  ```rust
  pub struct Adder{
    error:i32,
    position: u8,
    input: Option<(i32,i32)>
  }
  ```
  
- `Multiplier` simula il componente che opera le operazioni di moltiplicazione e divisione in un sistema di elaborazione
```rust
  pub struct Multiplier{
    error:i32,
    position: u8,
    input: Option<(i32,i32)>
  }
```

## Metodi Principali
La libreria contiene i seguenti metodi principali:
- ### Metodi del Builder
  - Metodi di `SnnBuilder`:
    - **new()**:
    ```rust
    pub fn new() -> Self
    ```
    crea un nuovo `SnnBuilder`

    - **add_weight()**:
    ```rust
    pub fn add_weight<const NUM_NEURONS: usize, const INPUT_DIM: usize >(&mut self, weights:[[f64; INPUT_DIM]; NUM_NEURONS]) -> &mut SnnBuilder<N> 
    ```
    aggiunge i pesi dal precedente layer al nuovo layer
  
    - **add_neurons()**:
    ```rust
    pub fn add_neurons<const NUM_NEURONS: usize>(&mut self, neurons: [N; NUM_NEURONS]) -> &mut SnnBuilder<N> 
    ```
    aggiunge neuroni al layer corrente
  
    - **add_intra_weight()**:
    ```rust
    pub fn add_intra_weights<const NUM_NEURONS: usize>(&mut self, intra_weights: [[f64; NUM_NEURONS]; NUM_NEURONS]) -> &mut SnnBuilder<N> 
    ```
    aggiunge i pesi tra i vari neuroni dello stesso layer

    - **build()**:
    ```rust
    pub fn build<const INPUT_DIM: usize, const OUTPUT_DIM:usize>(&mut self, components: &Vec<i32>, error_type: i32, info_table: &mut InfoTable) -> SNN<N, { INPUT_DIM }, { OUTPUT_DIM }>
    ```
    costruisce la `SNN` dalle informazioni raccolte fino a quel punto dal `SnnBuilder`. Il parametro `error_type` serve per forzare un errore specifico all'interno della rete.
  
- ### Metodi della Rete
  - Metodi di `Snn`:
    - **process()**:
    ```rust
    pub fn process<const SPIKES_DURATION: usize>(&mut self, input_spikes: &[[u8; SNN_INPUT_DIM]; SPIKES_DURATION])
                                                 -> [[u8; SNN_OUTPUT_DIM]; SPIKES_DURATION] 
    ```
    processa gli impulsi di input passati come parametri e ritorna gli impulsi di output della rete
- ### Metodi della Gestione dell'errore
  - Metodi di `Error Handling`:
    - **embed_error()**:
    ```rust
    fn embed_error(variable:f64, error:ErrorType, info_table: &mut InfoTable)->f64
    ```
    restituisce il valore di errore di `variable` dopo aver subito un errore di tipo `error`. 

## Esempio di Utilizzo
L'esempio seguente mostra come allocare *staticamente* una `Spiking Neural Network` usando `SnnBuilder`, e come eseguirlo avendo 
un output di 3 instanti per neurone.

```rust
    let mut binding = SnnBuilder::new();
    let builder = binding.add_layer().add_weight([
        [0.1, 0.2, 0.5],
        [0.3, 0.4, 0.2],
        [0.5, 0.6, 0.1]
    ]).add_neurons([
                        LIFNeuron::new(0.03, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.05, 0.05, 0.1, 1.0, 1.0),
                        LIFNeuron::new(0.09, 0.05, 0.1, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25, -0.3],
        [-0.10, 0.0, -0.3],
        [-0.1, -0.3,  0.0]
    ])
    .add_layer()
        .add_weight([
            [0.1, 0.2, 0.3],
            [0.4, 0.5, 0.6]
        ]).add_neurons([
        LIFNeuron::new(0.07, 0.04, 0.4, 1.0, 1.0),
        LIFNeuron::new(0.3, 0.01, 0.4, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],
        [-0.10, 0.0]
    ]).add_layer().add_weight([
        [0.1, 0.2],
        [0.3, 0.4],
        [0.5, 0.6]
    ]).add_neurons([
        LIFNeuron::new(0.03, 0.01, 0.1, 1.0, 1.0),
        LIFNeuron::new(0.05, 0.03, 0.2, 1.0, 1.0),
        LIFNeuron::new(0.09, 0.06, 0.4, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25, -0.3],
        [-0.10, 0.0, -0.3],
        [-0.1, -0.3,  0.0]
    ]).add_layer()
        .add_weight([
            [0.1, 0.2, 0.3],
            [0.4, 0.5, 0.6]
        ]).add_neurons([
        LIFNeuron::new(0.07, 0.01, 0.2, 1.0, 1.0),
        LIFNeuron::new(0.03, 0.08, 0.3, 1.0, 1.0),
    ]).add_intra_weights([
        [0.0, -0.25],
        [-0.10, 0.0]
    ]);

    let input = [[0,1,1], [0,0,1], [1,1,1]];


    /* SNN WITHOUT ANY ERROR */
    let mut snn_0_error = builder.clone().build::<3,2>(&Vec::new(), -1, &mut table);
    let snn_result_0_error= snn_0_error.process(&input);
    /* SNN WITH ERRORS */
    for _ in 0..n_faults {
        let mut snn = builder.clone().build::<3,2>(&components, error_index, &mut table);
        let snn_result= snn.process(&input);
}
```