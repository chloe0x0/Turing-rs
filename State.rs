
/*
 *
 *  Turing Machine
 *      => Current State
 *      => Transition function (state -> tuple[state, symbol, movement])
 *
 *
 *
 *
 *
 *
 */

pub struct State {
    sym: String,
    halt: bool
}

impl State {
    fn new(s: String, is_halt: bool) -> Self {
        State { sym: s, halt: is_halt }
    }
}


fn main() {
    println!("Hello World!");
}
