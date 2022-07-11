
#[derive(Debug)]
pub struct Tape {
    pub tape: Vec<String>
}

impl Tape {
    pub fn new(init_capacity: usize, empty_str: String) -> Self {
        Tape { tape: vec![empty_str; init_capacity] }
    }

    pub fn at(&self, index: usize) -> String {
        return self.tape[index].clone();
    }

    pub fn set(&mut self, index: usize, sym: String) {
        self.tape[index] = sym;
    }
}

