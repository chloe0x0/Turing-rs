
#[derive(Copy, Debug, Clone)]
pub enum Direction {
    LEFT,
    NONE,
    RIGHT,
}

impl Direction {
    pub fn from_str(string: &str) -> Self {
        match &string.to_lowercase() as &str {
            "l" => Direction::LEFT,
            "r" => Direction::RIGHT,
            "n" => Direction::NONE,
            _ => todo!(),
        }
    }
}
