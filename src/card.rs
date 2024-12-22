use crate::suit::Suit;

#[derive(Clone, Debug, PartialEq, PartialOrd, Eq, Hash)]
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