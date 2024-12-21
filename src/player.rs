#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    IP,
    OOP,
}

impl Player {
    pub fn as_usize(&self) -> usize {
        match self {
            Player::IP => 0,
            Player::OOP => 1,
        }
    }   
}