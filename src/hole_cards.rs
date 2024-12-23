use crate::card::Card;
use crate::suit::Suit;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct HoleCards {
    cards: [Card; 2],
}

impl PartialOrd for HoleCards {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_highest_card = std::cmp::max(self.cards[0].rank, self.cards[1].rank);
        let other_highest_card = std::cmp::max(other.cards[0].rank, other.cards[1].rank);

        self_highest_card.partial_cmp(&other_highest_card)
    }
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

    pub fn highest(&self) -> u8 {
        std::cmp::max(self.cards[0].rank, self.cards[1].rank)
    }

    pub fn cards(&self) -> [Card; 2] {
        self.cards.clone()
    }

}

impl fmt::Display for HoleCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}{:}", self.cards[0], self.cards[1])
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
        assert_eq!(hole_cards.cards, [card1, card2]);
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