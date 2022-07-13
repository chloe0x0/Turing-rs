
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct State {
    pub sym: String,
    pub halt: bool
}

impl State {
    pub fn new(s: &str, is_halt: bool) -> Self {
        State { sym: s.to_string(), halt: is_halt }
    }
}
