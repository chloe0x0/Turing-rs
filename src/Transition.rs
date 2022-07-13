// (Next_State, Write_Symbol, Direction)
//#[derive(Clone)]
#[path = "state.rs"]
mod state;
#[path = "direction.rs"]
mod direction;
use state::State;
use direction::Direction;

#[derive(Debug, Clone)]
pub struct Trans {
    pub next_state: State,
    pub write_sym: String,
    pub dir: Direction
}

impl Trans {
    pub fn new(s: State, sym: &str, d: Direction) -> Self {
        Trans {next_state: s, write_sym: sym.to_string(), dir: d}
    }
}    
