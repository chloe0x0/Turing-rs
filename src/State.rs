
#[derive(Debug)]
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
    let s: State = State::new(String::from("HALT"), true); 
    println!("{:?}", s);
}
