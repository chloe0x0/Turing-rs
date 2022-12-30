# Turing-rs 

Basic Turing Machine simulation in Rust

## 4 State Busy Beaver
```rust
use Turing::*;
use std::collections::{HashMap, HashSet};

fn main() {
    let halt: HashSet<String> = HashSet::from(["H".to_string()]);

    let trans: HashMap<(String, String), Trans> = HashMap::new();

    let tape_length: usize = 50;

    let mut T = TmDet::new("A", tape_length, "0", trans, halt);

    T.parse("A 0 B 1 R");
    T.parse("A 1 B 1 L");
    T.parse("B 0 A 1 L");
    T.parse("B 1 C 0 L");
    T.parse("C 1 D 1 L");
    T.parse("C 0 H 1 R");
    T.parse("D 1 A 0 R");
    T.parse("D 0 D 1 R");

    println!("{:?}", T.run(true, 250, 15));
}
```

## Current Functionality
* Simulation of Determinstic Turing Machines on a single track, extendable tape


## Potential TODO
* Implement Non-deterministic Turing Machines
 
* Two Dimensional Turing Machines

* Probabalistic Turing Machines
 
