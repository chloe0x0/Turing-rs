mod state;
mod direction;
mod transition;
mod tape;
use std::collections::HashMap;

use transition::Trans;
use tape::Tape;
use state::State;
use direction::Direction;

// Reports the Status of a Turing Machine time step
enum Status<T> {
    Err(T),  // Error message 
    Warn(T), // warning 
    Valid
}

#[derive(Debug)]
pub struct TM {
    state: State,
    pos: usize,      
    tape: Tape,
    trans_fun: HashMap<(State, String), Trans> // Map (state::State, tape[pos]) => transition tuple
}

impl TM {
    fn new(s0: State, ln: usize, em: String, tr: HashMap<(State, String), Trans>) -> Self {
        TM { state: s0, tape: Tape::new(ln, em), pos: ln / 2, trans_fun: tr }
    }

    fn iter(&mut self) -> Status<String> {
        if self.state.halt {
            return Status::Warn(String::from("Machine is halted!"));
        }
        // Read the head symbol
        let s: String = self.tape.at(self.pos);

        // construct the machine tuple
        let machine_tuple: (State, String) = (self.state.clone(), s.clone());
        // If the machine tuple is not contained in the domain of the transition function panic
        if !self.trans_fun.contains_key(&machine_tuple) {
            // Current state::State is undefined
            let st: String = format!("The transition for state '{}' when '{}' is read is never defined'", self.state.sym, s);
            return Status::Err(st);
        }
        // Compute the transition
        let t: &Trans =  &*self.trans_fun.get(&machine_tuple).unwrap();

        // Update state::State
        self.state = t.next_state.clone();

        // Get write symbol
        let write_s: String = t.write_sym.clone();
        // Write the symbol
        self.tape.set(self.pos, write_s);

        // Update the head position
        self.pos += t.dir as usize;

        // valid iteration
        return Status::Valid;
    }
}

fn main() {
    println!("Hello World! ");
}

