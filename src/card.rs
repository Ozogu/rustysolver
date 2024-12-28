use crate::suit::Suit;
use std::fmt;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash, Ord)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        Card { rank, suit }
    }

    pub fn new_with_rank(rank: u8) -> Self {
        Card { rank, suit: Suit::Diamonds }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let rank_str = self.rank.to_string();
        let r = match self.rank {
            14 => "A",
            13 => "K",
            12 => "Q",
            11 => "J",
            10 => "T",
            _ => &rank_str,
        };
        write!(f, "{:}{:}", r, self.suit)
    }
}