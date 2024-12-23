use crate::hole_cards::HoleCards;
use crate::player::Player;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hole_cards::HoleCards;

    #[test]
    fn test_player_cards() {
        let ip = HoleCards::new_with_rank(0);
        let oop = HoleCards::new_with_rank(1);
        let player_cards = PlayerCards::new(ip.clone(), oop.clone());
        assert_eq!(player_cards.get(Player::IP), ip);
        assert_eq!(player_cards.get(Player::OOP), oop);
    }
}