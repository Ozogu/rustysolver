use crate::card::Card;
use crate::suit::Suit;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct HoleCards {
    cards: [Card; 2],
}

impl HoleCards {
    pub fn new(card1: &Card, card2: &Card) -> Self {
        HoleCards {
            cards: [card1.clone(), card2.clone()],
        }
    }

    pub fn new_with_ranks(rank1: u8, rank2: u8) -> Self {
        let card1 = Card::new(rank1, Suit::Diamonds);
        let card2 = Card::new(rank2, Suit::Diamonds);
        HoleCards {
            cards: [card1, card2],
        }
    }

    pub fn new_with_rank(rank: u8) -> Self {
        let card = Card::new(rank, Suit::Diamonds);
        HoleCards {
            cards: [card.clone(), card],
        }
    }

    pub fn cards(&self) -> [Card; 2] {
        self.cards.clone()
    }
}