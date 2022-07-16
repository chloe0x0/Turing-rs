// (Next_State, Write_Symbol, Direction)
//#[derive(Clone)]
#[path = "direction.rs"]
mod direction;
use direction::Direction;

#[derive(Debug, Clone)]
pub struct Trans {
    pub next_state: String,
    pub write_sym: String,
    pub dir: Direction
}

impl Trans {
    pub fn new(s: &str, sym: &str, d: Direction) -> Self {
        Trans {next_state: s.to_string(), write_sym: sym.to_string(), dir: d}
    }
}    
