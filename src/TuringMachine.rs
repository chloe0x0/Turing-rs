mod state;
mod direction;
mod transition;
mod tape;
use std::collections::HashMap;
use std::collections::HashSet;
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
    alpha: HashSet<String>,
    state_space: HashSet<State>,
    trans_fun: HashMap<(State, String), Trans> // Map (state::State, tape[pos]) => transition tuple
}

impl TM {
    fn new(s0: State, ln: usize, em: &str, tr: HashMap<(State, String), Trans>, alpha: HashSet<String>, state_space: HashSet<State>) -> Self {
        TM { state: s0, tape: Tape::new(ln, em), pos: ln / 2, trans_fun: tr, alpha: alpha, state_space: state_space }
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

    // Validates transition function
    fn validate_trans(&self) -> Result<(), String> {
        let mut halt_states: usize = 0;

        for (tp, tr) in &self.trans_fun {
            if !self.alpha.contains(&tr.write_sym) {
                return Err(format!("'{}' is not contained in the machine's alphabet", tr.write_sym));
            }
            else if !self.alpha.contains(&tp.1) {
                return Err(format!("'{}' is not contained in the machine's alphabet", tp.1));
            }

            for symbol in &self.alpha {
                let t: (State, String) = (tp.0.clone(), symbol.clone());
                if !self.trans_fun.contains_key(&t) {
                    return Err(format!("State {} has an undefined transition when '{}' is read", tp.0.sym, symbol));
                }
            }
           
            halt_states += tr.next_state.halt as usize;
        }
        
        match halt_states == 0 {
            true => Err("Transition function has no halting state!".to_string()),
            false => Ok(())
        }
    }
}

fn main() {
    // 2 State Busy Beaver
    // a0 -> b1r    a1 -> b1l
    // b0 -> a1l    b1 -> h1r
    let A: State = State::new("A", false);
    let B: State = State::new("B", false);
    let C: State = State::new("C", false);
    let D: State = State::new("D", false);
    let H: State = State::new("H", true);

    let alpha = HashSet::from(["1".to_string(), "0".to_string()]);
    let state_space = HashSet::from([A.clone(), B.clone(), C.clone(), D.clone(), H.clone()]);

    let trans: HashMap<(State, String), Trans> = HashMap::from([
        ( (A.clone(), "1".to_string()), Trans::new(B.clone(), "1", Direction::LEFT)),
        ( (A.clone(), "0".to_string()), Trans::new(B.clone(), "1", Direction::RIGHT)),
        ( (B.clone(), "0".to_string()), Trans::new(A.clone(), "1", Direction::LEFT)),
        ( (B.clone(), "1".to_string()), Trans::new(C.clone(), "0", Direction::LEFT)),
        ( (C.clone(), "1".to_string()), Trans::new(D.clone(), "1", Direction::LEFT)),
        ( (C.clone(), "0".to_string()), Trans::new(H.clone(), "1", Direction::RIGHT)),
        ( (D.clone(), "1".to_string()), Trans::new(A.clone(), "0", Direction::RIGHT)),
        ( (D.clone(), "0".to_string()), Trans::new(D.clone(), "1", Direction::RIGHT)),
    ]);

    let tape_length: usize = 25;
    
    let mut T = TM::new(A.clone(), tape_length, "0", trans, alpha, state_space);

    if !T.validate_trans().is_ok() {
        panic!("{:?}", T.validate_trans());
    }

    while true{
        
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

        if T.state.halt {
            break;
        }
    }
}

