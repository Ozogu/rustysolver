use crate::card::Card;
use crate::suit::Suit;

use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HoleCards {
    pub card1: Card,
    pub card2: Card,
}

impl HoleCards {
    pub fn new(card1: &Card, card2: &Card) -> Self {
        let suit1 = card1.suit;
        let suit2 = card2.suit;
        if suit1 == Suit::Offsuit || suit2 == Suit::Offsuit ||
              suit1 == Suit::Suited || suit2 == Suit::Suited {
            assert!(card1.suit == card2.suit, "Cards must have the same suit");
        }

        if card1 >= card2 {
            HoleCards {
                card1: card1.clone(),
                card2: card2.clone(),
            }
        } else {
            HoleCards {
                card1: card2.clone(),
                card2: card1.clone(),
            }
        }
    }

    pub fn new_with_ranks(rank1: u8, rank2: u8) -> Self {
        let card1 = Card::new(rank1, Suit::Diamonds);
        let card2 = Card::new(rank2, Suit::Diamonds);
        HoleCards::new(&card1, &card2)
    }

    pub fn new_with_rank(rank: u8) -> Self {
        let card = Card::new(rank, Suit::Offsuit);
        HoleCards::new(&card, &card)
    }

    pub fn highest(&self) -> u8 {
        std::cmp::max(self.card1.rank, self.card2.rank)
    }

    pub fn cards(&self) -> [Card; 2] {
        [self.card1.clone(), self.card2.clone()]
    }

    pub fn expand(&self) -> Vec<HoleCards> {
        let suit1 = self.card1.suit;
        let suit2 = self.card2.suit;
        if suit1 == Suit::Offsuit && suit2 == Suit::Offsuit {
            self.expand_offsuit()
        } else if suit1 == Suit::Suited && suit2 == Suit::Suited {
            self.expand_suited()
        } else {
            vec![self.clone()]
        }
    }

    fn expand_offsuit(&self) -> Vec<HoleCards> {
        let mut hole_cards = Vec::new();
        for s1 in Suit::to_vec() {
            for s2 in Suit::to_vec() {
                if s1 != s2 {
                    hole_cards.push(HoleCards::new(
                        &Card::new(self.card1.rank, s1),
                        &Card::new(self.card2.rank, s2),
                    ));
                }
            }
        }

        hole_cards
    }

    fn expand_suited(&self) -> Vec<HoleCards> {
        let mut hole_cards = Vec::new();
        for s in Suit::to_vec() {
            hole_cards.push(HoleCards::new(
                &Card::new(self.card1.rank, s),
                &Card::new(self.card2.rank, s),
            ));
        }

        hole_cards
    }

    pub fn new_from_string(hole_cards_str: &str) -> Self {
        if hole_cards_str.len() == 2 && hole_cards_str.chars().nth(0) == hole_cards_str.chars().nth(1) {
            let rank = Card::rank_from_char(hole_cards_str.chars().next().unwrap());
            return HoleCards::new_with_rank(rank);
        } else if hole_cards_str.len() == 3 {
            let rank1 = Card::rank_from_char(hole_cards_str.chars().next().unwrap());
            let rank2 = Card::rank_from_char(hole_cards_str.chars().nth(1).unwrap());
            let suit_char = hole_cards_str.chars().nth(2).unwrap();
            let suit = match suit_char {
                's' => Suit::Suited,
                'o' => Suit::Offsuit,
                _ => panic!("Invalid suit character: {}", suit_char),
            };
            return HoleCards::new(&Card::new(rank1, suit), &Card::new(rank2, suit));
        } else if hole_cards_str.len() == 4 {
            let rank1 = Card::rank_from_char(hole_cards_str.chars().next().unwrap());
            let suit1 = Suit::from_char(hole_cards_str.chars().nth(1).unwrap());
            let rank2 = Card::rank_from_char(hole_cards_str.chars().nth(2).unwrap());
            let suit2 = Suit::from_char(hole_cards_str.chars().nth(3).unwrap());
            return HoleCards::new(&Card::new(rank1, suit1), &Card::new(rank2, suit2));
        } else {
            panic!("Invalid hole cards string: {}", hole_cards_str);
        }
    }

}

impl fmt::Display for HoleCards {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}{:}", self.card1, self.card2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let card1 = Card::new(2, Suit::Diamonds);
        let card2 = Card::new(1, Suit::Diamonds);
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
        assert_eq!(hole_cards1, hole_cards2);
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

    #[test]
    fn test_ahad_from_string() {
        let hole_cards_str = "AhAd";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(14, Suit::Hearts));
        assert_eq!(hole_cards.card2, Card::new(14, Suit::Diamonds));
    }

    #[test]
    fn test_ahkh_from_string() {
        let hole_cards_str = "AhKh";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(14, Suit::Hearts));
        assert_eq!(hole_cards.card2, Card::new(13, Suit::Hearts));
    }

    #[test]
    fn test_aa_from_string() {
        let hole_cards_str = "AA";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(14, Suit::Offsuit));
        assert_eq!(hole_cards.card2, Card::new(14, Suit::Offsuit));
    }

    #[test]
    fn test_aks_from_string() {
        let hole_cards_str = "AKs";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(14, Suit::Suited));
        assert_eq!(hole_cards.card2, Card::new(13, Suit::Suited));
    }

    #[test]
    fn test_72o_from_string() {
        let hole_cards_str = "72o";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(7, Suit::Offsuit));
        assert_eq!(hole_cards.card2, Card::new(2, Suit::Offsuit));
    }

    #[test]
    fn test_AsKs_from_string() {
        let hole_cards_str = "AsKs";
        let hole_cards = HoleCards::new_from_string(hole_cards_str);
        assert_eq!(hole_cards.card1, Card::new(14, Suit::Spades));
        assert_eq!(hole_cards.card2, Card::new(13, Suit::Spades));
    }

    #[test]
    #[should_panic(expected = "Cards must have the same suit")]
    fn test_creating_invalid_hole_cards() {
        let card1 = Card::new(2, Suit::Offsuit);
        let card2 = Card::new(1, Suit::Hearts);
        HoleCards::new(&card1, &card2);
    }

    #[test]
    fn test_expand_offsuit() {
        let hole_cards = HoleCards::new(
            &Card::new(2, Suit::Offsuit),
            &Card::new(1, Suit::Offsuit),
        );
        let expanded_hole_cards = hole_cards.expand();
        assert_eq!(expanded_hole_cards.len(), 12);
        for cards in expanded_hole_cards {
            assert!(cards.card1.suit != cards.card2.suit);
            assert!(cards.card1.suit != Suit::Offsuit);
            assert!(cards.card2.suit != Suit::Offsuit);
            assert!(cards.card1.suit != Suit::Suited);
            assert!(cards.card2.suit != Suit::Suited);
        }
    }

    #[test]
    fn test_expand_suited() {
        let hole_cards = HoleCards::new(
            &Card::new(2, Suit::Suited),
            &Card::new(1, Suit::Suited),
        );
        let expanded_hole_cards = hole_cards.expand();
        assert_eq!(expanded_hole_cards.len(), 4);
        for cards in expanded_hole_cards {
            assert_eq!(cards.card1.suit, cards.card2.suit);
            assert!(cards.card1.suit != Suit::Offsuit);
            assert!(cards.card2.suit != Suit::Offsuit);
            assert!(cards.card1.suit != Suit::Suited);
            assert!(cards.card2.suit != Suit::Suited);
        }
    }
}