use crate::card::Card;
use crate::suit::Suit;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HoleCards {
    pub card1: Card,
    pub card2: Card,
}

impl PartialOrd for HoleCards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_highest_card = std::cmp::max(self.card1.rank, self.card2.rank);
        let other_highest_card = std::cmp::max(other.card1.rank, other.card2.rank);

        self_highest_card.partial_cmp(&other_highest_card)
    }
}

impl HoleCards {
    pub fn new(card1: &Card, card2: &Card) -> Self {
        HoleCards {
            card1: card1.clone(),
            card2: card2.clone(),
        }
    }

    pub fn new_with_ranks(rank1: u8, rank2: u8) -> Self {
        let card1 = Card::new(rank1, Suit::Diamonds);
        let card2 = Card::new(rank2, Suit::Diamonds);
        HoleCards {
            card1,
            card2,
        }
    }

    pub fn new_with_rank(rank: u8) -> Self {
        let card = Card::new(rank, Suit::Diamonds);
        HoleCards {
            card1: card.clone(),
            card2: card,
        }
    }

    pub fn highest(&self) -> u8 {
        std::cmp::max(self.card1.rank, self.card2.rank)
    }

    pub fn cards(&self) -> [Card; 2] {
        [self.card1.clone(), self.card2.clone()]
    }

}

impl fmt::Display for HoleCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}{:}", self.card1, self.card2)
    }
}

impl Ord for HoleCards {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let card1 = Card::new(1, Suit::Diamonds);
        let card2 = Card::new(2, Suit::Diamonds);
        let hole_cards = HoleCards::new(&card1, &card2);
        assert_eq!(hole_cards.card1, card1);
        assert_eq!(hole_cards.card2, card2);
    }

    #[test]
    fn test_cmp_same_hole_cards() {
        let hole_cards1 = HoleCards::new_with_ranks(1, 2);
        let hole_cards2 = HoleCards::new_with_ranks(1, 2);
        assert_eq!(hole_cards1 < hole_cards2, false);
    }

    #[test]
    fn test_cmp_same_hole_cards_different_order() {
        let hole_cards1 = HoleCards::new_with_ranks(1, 2);
        let hole_cards2 = HoleCards::new_with_ranks(2, 1);
        assert_eq!(hole_cards1 < hole_cards2, false);
    }

    #[test]
    fn test_cmp_different_hole_cards() {
        let hole_cards1 = HoleCards::new_with_ranks(1, 2);
        let hole_cards2 = HoleCards::new_with_ranks(2, 3);
        assert_eq!(hole_cards1 < hole_cards2, true);
    }

    #[test]
    fn test_cmp_pocket_pairs() {
        let hole_cards1 = HoleCards::new_with_ranks(1, 1);
        let hole_cards2 = HoleCards::new_with_ranks(2, 2);
        assert_eq!(hole_cards1 < hole_cards2, true);
    }
}