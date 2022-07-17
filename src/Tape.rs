use std::collections::VecDeque;
use std::convert::TryInto;

const BUFFER_LEN: usize = 25;

#[derive(Debug)]
pub struct Tape {
    pub tape: VecDeque<String>,
    pub empty_str: String,
}

impl Tape {
    pub fn new(init_capacity: usize, empty_str: &str) -> Self {
        Tape { tape: VecDeque::from(vec![empty_str.to_string(); init_capacity]), empty_str: empty_str.to_string() }
    }

    fn extend_back(&mut self) { 
        for n in 0..BUFFER_LEN {
            self.tape.push_back(self.empty_str.clone());
        }
    }

    fn extend_front(&mut self) {
        for n in 0..BUFFER_LEN {
            self.tape.push_front(self.empty_str.clone());
        }
    }

    pub fn at(&mut self, index: i64) -> String {
        if index < 0 { 
            self.extend_front();
            return self.tape[0].clone();
        }
        else if index > (self.tape.len()-1).try_into().unwrap() {
            self.extend_back();
            let len = self.tape.len();
            return self.tape[len - 1].clone();
        }
        return self.tape[index as usize].clone(); 
    }

    pub fn set(&mut self, index: i64, sym: String) {
        if index < 0 {
            self.extend_front();
            self.tape[0] = sym;
        }
        else if index > (self.tape.len()-1).try_into().unwrap() {
            self.extend_back();
            let len = self.tape.len();
            self.tape[len - 1] = sym;
        }
        else {
            self.tape[index as usize] = sym;
        }
    }
}

