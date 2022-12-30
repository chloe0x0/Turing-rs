use std::collections::VecDeque;
use std::convert::TryInto;

const BUFFER_LEN: usize = 25;

#[derive(Debug, Clone)]
pub struct Tape {
    pub tape: VecDeque<String>,
    pub empty_str: String,
}

impl Tape {
    pub fn new(init_capacity: usize, empty_str: &str) -> Self {
        Tape {
            tape: VecDeque::from(vec![empty_str.to_string(); init_capacity]),
            empty_str: empty_str.to_string(),
        }
    }

    /// Extend the back of the tape
    fn extend_back(&mut self) {
        for _ in 0..BUFFER_LEN {
            self.tape.push_back(self.empty_str.clone());
        }
    }

    /// Extend the front of the tape
    fn extend_front(&mut self) {
        for _ in 0..BUFFER_LEN {
            self.tape.push_front(self.empty_str.clone());
        }
    }

    /// Get the symbol at a cell
    pub fn at(&mut self, index: i64) -> String {
        if index < 0 {
            self.extend_front();
            return self.tape[0].clone();
        } else if index > (self.tape.len() - 1).try_into().unwrap() {
            self.extend_back();
            let len = self.tape.len();
            return self.tape[len - 1].clone();
        }
        return self.tape[index as usize].clone();
    }

    /// Write a symbol to a cell
    pub fn set(&mut self, index: i64, sym: String) {
        if index < 0 {
            self.extend_front();
            self.tape[0] = sym;
        } else if index > (self.tape.len() - 1).try_into().unwrap() {
            self.extend_back();
            let len = self.tape.len();
            self.tape[len - 1] = sym;
        } else {
            self.tape[index as usize] = sym;
        }
    }

    /// Print the tape at a certain position with a certain width around that position
    /// [0, 0, 0, 0, 0, 0, 0, 1]
    /// pos: 3
    /// width = 2
    /// would print 2 cells to the left of position 3 and 2 cells to the right
    pub fn print_at(&self, pos: i64, width: i64) {
        let start: usize = (pos - width).try_into().unwrap();
        let end: usize = (pos + width).try_into().unwrap();

        let mut s = String::new();

        for i in start..=end {
            let symbol = &self.tape[i].clone();
         
            match symbol.eq(&self.empty_str) {
                true => s.push(' '),
                false => s.push_str(symbol)
            }
        }

        println!("{}", s);
    }

}
