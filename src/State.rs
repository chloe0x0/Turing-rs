
#[derive(Eq, Hash, PartialEq, Debug, Clone)]
pub struct State {
    pub sym: String,
    pub halt: bool
}

impl State {
    pub fn new(s: String, is_halt: bool) -> Self {
        State { sym: s, halt: is_halt }
    }
}
