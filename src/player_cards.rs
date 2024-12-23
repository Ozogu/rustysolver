use crate::hole_cards::HoleCards;
use crate::player::Player;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PlayerCards {
    ip: HoleCards,
    oop: HoleCards,
}

impl PlayerCards {
    pub fn new(ip: HoleCards, oop: HoleCards) -> Self {
        PlayerCards {
            ip,
            oop,
        }
    }

    pub fn get(&self, player: Player) -> HoleCards {
        match player {
            Player::IP => self.ip.clone(),
            Player::OOP => self.oop.clone(),
        }
    }
}

impl fmt::Display for PlayerCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}|{:}", self.ip, self.oop)
    }
}