use crate::{direction::Direction, tape::Tape, transition::Trans};
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::{self, Write};

// Reports the Status of a Turing Machine time step
#[derive(Debug)]
pub enum Status<T> {
    Err(T),  // Error message
    Warn(T), // warning
    Valid,
}

impl<T> Status<T> {
    #[inline]
    fn is_err(&self) -> bool {
        matches!(*self, Status::Err(_))
    }
}

#[derive(Debug)]
pub struct TmDet {
    state: String,
    pos: i64,
    pub tape: Tape,
    halt_set: HashSet<String>,
    trans_fun: HashMap<(String, String), Trans>, // Map (state::State, tape[pos]) => transition tuple
}

impl TmDet {
    pub fn new(
        s0: &str,
        ln: usize,
        em: &str,
        tr: HashMap<(String, String), Trans>,
        halt: HashSet<String>,
    ) -> Self {
        TmDet {
            state: s0.to_string(),
            tape: Tape::new(ln, em),
            pos: ln as i64 / 2,
            trans_fun: tr,
            halt_set: halt,
        }
    }

    fn move_head(&mut self, d: Direction) {
        match d {
            Direction::LEFT => self.pos -= 1,
            Direction::NONE => return,
            Direction::RIGHT => self.pos += 1,
        }
    }

    pub fn iter(&mut self) -> Status<String> {
        if self.halt_set.contains(&self.state) {
            return Status::Warn("Machine is halted!".to_string());
        }
        // Read the head symbol
        let s: String = self.tape.at(self.pos);

        // construct the machine tuple
        let machine_tuple: (String, String) = (self.state.clone(), s.clone());
        // If the machine tuple is not contained in the domain of the transition function panic
        if !self.trans_fun.contains_key(&machine_tuple) {
            // Current state::State is undefined
            let st: String = format!(
                "The transition for state '{}' when '{}' is read is never defined'",
                self.state, s
            );
            return Status::Err(st);
        }
        // Compute the transition
        let t: &Trans = &*self.trans_fun.get(&machine_tuple).unwrap();

        // Update state::State
        self.state = t.next_state.clone();

        // Get write symbol
        let write_s: String = t.write_sym.clone();
        // Write the symbol
        self.tape.set(self.pos, write_s);

        // Update the head position
        self.move_head(t.dir);

        // valid iteration
        Status::Valid
    }

    pub fn parse(&mut self, trans: &str) {
        // Format is: State Read State Write Dir
        let tokens: Vec<&str> = trans.split(" ").collect();
        assert_eq!(tokens.len(), 5);

        let dir: Direction = Direction::from_str(tokens[4]);
        self.trans_fun.insert(
            (tokens[0].to_string(), tokens[1].to_string()),
            Trans::new(tokens[2], tokens[3], dir),
        );
    }

    /// Run the Machine until it halts (if it halts that is)
    pub fn run(&mut self, verbose: bool, max_iters: i128, width: i64) -> Status<String> {
        let mut acc: i128 = 0;
        
        while !self.halt_set.contains(&self.state) {
            let stat: Status<String> = self.iter();

            if verbose {
                self.tape.print_at(self.pos, width);
            }

            acc += 1;            

            if acc == max_iters {
                return Status::Warn("Machine ran until max iters without halting. Aborting run".to_string());
            } 

            if stat.is_err() {
                return stat;
            }
        }

        return Status::Valid;
    }
}

