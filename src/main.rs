use Turing::*;
use std::collections::{HashMap, HashSet};

fn main() {
    // 2 State Busy Beaver
    // a0 -> b1r    a1 -> b1l
    // b0 -> a1l    b1 -> h1r
    let halt: HashSet<String> = HashSet::from(["H".to_string()]);

    let trans: HashMap<(String, String), Trans> = HashMap::new();

    let tape_length: usize = 50;

    let mut T = TmDet::new("A", tape_length, "0", trans, halt);

    // Current State Head Symbol Next State Write Symbol Direction
    // 4 state busy beaver

    T.parse("A 0 B 1 R");
    T.parse("A 1 B 1 L");
    T.parse("B 0 A 1 L");
    T.parse("B 1 C 0 L");
    T.parse("C 1 D 1 L");
    T.parse("C 0 H 1 R");
    T.parse("D 1 A 0 R");
    T.parse("D 0 D 1 R");

    println!("{:?}", T.run(true, 250, 15));

    // Turing's first example

    /*
    T.parse("b . c 0 R");
    T.parse("c . e . R");
    T.parse("e . f 1 R");
    T.parse("f . b . R");
    */

    // Zeno Machine
    /*x
    T.parse("q0 0 q0 1 N");
    T.parse("q0 1 q0 0 N");
    */

    // Thue-Morse Sequence Generator
    // L system of the form: {0 -> 01; 1 -> 10}
    // 0 -> 01 -> 0110 -> 01101001 -> ...


}
