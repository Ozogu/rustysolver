use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq, Ord, PartialOrd)]
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

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Player::IP => write!(f, "IP"),
            Player::OOP => write!(f, "OOP"),
        }
    }
}