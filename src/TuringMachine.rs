mod state;
mod direction;
mod transition;
mod tape;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write};
use std::env;

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
    state: String,
    pos: usize,      
    pub tape: Tape,
    halt_set: HashSet<String>,
    trans_fun: HashMap<(String, String), Trans> // Map (state::State, tape[pos]) => transition tuple
}

impl TM {
    fn new(s0: &str, ln: usize, em: &str, tr: HashMap<(String, String), Trans>, halt: HashSet<String>) -> Self {
        TM { state: s0.to_string(), tape: Tape::new(ln, em), pos: ln / 2, trans_fun: tr,  halt_set: halt }
    }

    fn move_head(&mut self, d: Direction) {
        match d {
            Direction::LEFT => self.pos -= 1,
            Direction::NONE => return,
            Direction::RIGHT => self.pos += 1
        }
    }

    fn iter(&mut self) -> Status<String> {
        if self.halt_set.contains(&self.state) {
            return Status::Warn(String::from("Machine is halted!"));
        }
        // Read the head symbol
        let s: String = self.tape.at(self.pos);

        // construct the machine tuple
        let machine_tuple: (String, String) = (self.state.clone(), s.clone());
        // If the machine tuple is not contained in the domain of the transition function panic
        if !self.trans_fun.contains_key(&machine_tuple) {
            // Current state::State is undefined
            let st: String = format!("The transition for state '{}' when '{}' is read is never defined'", self.state, s);
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

    fn parse(&mut self, trans: &str) {
        // Format is: State Read State Write Dir
        let tokens: Vec<&str> = trans.split(" ").collect();

        let dir: Direction = Direction::from_str(tokens[4]);
        self.trans_fun.insert((tokens[0].to_string(), tokens[1].to_string()), Trans::new(tokens[2], tokens[3], dir));
    }

    fn run(&mut self) -> Status<String> {
        while !self.halt_set.contains(&self.state) {
            let stat: Status<String> = self.iter();

            if stat.is_err() {
                return stat;
            }
        }

        return Status::Valid;
    }
}

fn main() {
    // 2 State Busy Beaver
    // a0 -> b1r    a1 -> b1l
    // b0 -> a1l    b1 -> h1r
    let halt: HashSet<String> = HashSet::from(["H".to_string()]);

    let trans: HashMap<(String, String), Trans> = HashMap::new();

    let tape_length: usize = 5;
    
    let mut T = TM::new("b", tape_length, ".", trans, halt);

    // Current State Head Symbol Next State Write Symbol Direction
    // 4 state busy beaver
    /*
    T.parse("A 0 B 1 R");
    T.parse("A 1 B 1 L");
    T.parse("B 0 A 1 L");
    T.parse("B 1 C 0 L");
    T.parse("C 1 D 1 L");
    T.parse("C 0 H 1 R");
    T.parse("D 1 A 0 R");
    T.parse("D 0 D 1 R");
    */
    // Turing's first example
    T.parse("b . c 0 R");
    T.parse("c . e . R");
    T.parse("e . f 1 R");
    T.parse("f . b . R");

    for i in 0..100 {
        
        print!("{} \t", T.state);

        for n in T.tape.tape.iter() {
            if n == &*T.tape.empty_str { print!(" "); }
            else { print!("{}", n); }
        }
        print!("|||||");
        io::stdout().flush().unwrap();
        println!("");

        let stat: Status<String> = T.iter();
        if stat.is_err() {
            println!("{:?}", stat);
            break;
        }

        if T.halt_set.contains(&T.state) {
            break;
        }
    }
}

