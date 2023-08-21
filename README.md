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
## Organizzazione
La libreria è organizzata come segue:
- ### Builder
  Il modulo `Builder` permette di creare la struttura della rete con i rispettivi layers
- ### Rete
## Strutture Principali
La libreria provvede le seguenti strutture:

- `LIFNeuron` rappresenta il neurone per il modello `Leaky Integrate and Fire` e può venir utilizzato per creare un `Layer` di neuroni:

```rust
```

- `Layer` rappresenta uno strato di neuroni, viene utilizzato per costruire i layer della `SNN`.

```rust
```

- `Evento` rappresenta un evento di uno strato di neuroni che si attiva ad un determinato istante di tempo. Incapsula gli impulsi che passano attraverso la rete.
```rust
```
- `SNN`
- `Processor`
- `SNNBuilder`

## Metodi Principali
