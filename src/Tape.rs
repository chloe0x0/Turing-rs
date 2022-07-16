use std::collections::VecDeque;

const BUFFER_LEN: usize = 1;

#[derive(Debug)]
pub struct Tape {
    pub tape: VecDeque<String>,
    pub empty_str: String,
}

impl Tape {
    pub fn new(init_capacity: usize, empty_str: &str) -> Self {
        Tape { tape: VecDeque::from(vec![empty_str.to_string(); init_capacity]), empty_str: empty_str.to_string() }
    }

    pub fn extend_back(&mut self) { 
        for n in 0..BUFFER_LEN {
            self.tape.push_back(self.empty_str.clone());
        }
    }

    pub fn extend_front(&mut self) {
        for n in 0..BUFFER_LEN {
            self.tape.push_front(self.empty_str.clone());
        }
    }

    pub fn at(&mut self, index: usize) -> String {
        if index < 0 { 
            self.extend_front();
            return self.tape[index + BUFFER_LEN].clone();
        }
        else if index > self.tape.len()-1 {
            self.extend_back();
            return self.tape[index+1 - BUFFER_LEN].clone();
        }
        return self.tape[index].clone(); 
    }

    pub fn set(&mut self, index: usize, sym: String) {
        if index < 0 {
            self.extend_front();
            self.tape[index + BUFFER_LEN] = sym;
        }
        else if index > self.tape.len()-1 {
            self.extend_back();
            self.tape[index+1 - BUFFER_LEN] = sym;
        }
        else {
            self.tape[index] = sym;
        }
    }
}

