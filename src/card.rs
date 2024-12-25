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
        write!(f, "{:}{:}", self.rank, self.suit)
    }
}