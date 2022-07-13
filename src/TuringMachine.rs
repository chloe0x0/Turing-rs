mod state;
mod direction;
mod transition;
mod tape;
use std::collections::HashMap;
use std::io::{self, Write};

use transition::Trans;
use tape::Tape;
use state::State;
use direction::Direction;

// Reports the Status of a Turing Machine time step
#[derive(Debug)]
enum Status<T> {
    Err(T),  // Error message 
    Warn(T), // warning 
    Valid
}

impl<T> Status<T> {
    #[inline]
    fn is_err(&self) -> bool {
        matches!(*self, Status::Err(_))
    }
}

#[derive(Debug)]
pub struct TM {
    state: State,
    pos: usize,      
    pub tape: Tape,
    trans_fun: HashMap<(State, String), Trans> // Map (state::State, tape[pos]) => transition tuple
}

impl TM {
    fn new(s0: State, ln: usize, em: &str, tr: HashMap<(State, String), Trans>) -> Self {
        TM { state: s0, tape: Tape::new(ln, em), pos: ln / 2, trans_fun: tr }
    }

    fn move_head(&mut self, d: Direction) {
        match d {
            Direction::LEFT => self.pos -= 1,
            Direction::NONE => return,
            Direction::RIGHT => self.pos += 1
        }
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
        self.move_head(t.dir);
        // valid iteration
        return Status::Valid;
    }

    fn run(&mut self) -> Status<String> {
        while !self.state.halt {
            let stat: Status<String> = self.iter();

            if stat.is_err() {
                return stat;
            }
        }

        return Status::Valid;
    }

    // Validates the transition function
    fn validate_trans(&self) -> bool {
        for (tp, tr) in &self.trans_fun {
            if tr.next_state.halt {
                return true;
            }
        }

        return false;
    }
}

fn main() {
    // 2 State Busy Beaver
    // a0 -> b1r    a1 -> b1l
    // b0 -> a1l    b1 -> h1r
    let A: State = State::new("A", false);
    let B: State = State::new("B", false);
    let HALT: State = State::new("HALT", true);

    let trans: HashMap<(State, String), Trans> = HashMap::from([
        ( (A.clone(), "1".to_string()), Trans::new(B.clone(), "1", Direction::LEFT)),
        ( (A.clone(), "0".to_string()), Trans::new(B.clone(), "1", Direction::RIGHT)),
        ( (B.clone(), "0".to_string()), Trans::new(A.clone(), "1", Direction::LEFT)),
        ( (B.clone(), "1".to_string()), Trans::new(HALT.clone(), "1", Direction::RIGHT)),
    ]);

    let tape_length: usize = 25;


   let mut T = TM::new(A.clone(), tape_length, "0", trans);

    while !T.state.halt{
        
        print!("{} \t", T.state.sym);

        for n in T.tape.tape.iter() {
            if n == "0" { print!(" "); }
            else { print!("1"); }
        }
        io::stdout().flush().unwrap();
        println!("");

        let stat: Status<String> = T.iter();
        if stat.is_err() {
            println!("{:?}", stat);
            break;
        }
    }

}

