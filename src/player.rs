#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
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

    pub fn opponent(&self) -> Player {
        match self {
            Player::IP => Player::OOP,
            Player::OOP => Player::IP,
        }
    }
}