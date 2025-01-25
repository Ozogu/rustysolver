use crate::suit::Suit;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Card {
    pub suit: Suit,
    pub rank: u8,
}

impl Card {
    pub fn new(rank: u8, suit: Suit) -> Self {
        assert!(rank >= 1 && rank <= 14);
        Card { rank, suit }
    }

    pub fn new_with_rank(rank: u8) -> Self {
        assert!(rank >= 1 && rank <= 14);
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

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.rank < other.rank {
            Some(std::cmp::Ordering::Less)
        } else if self.rank > other.rank {
            Some(std::cmp::Ordering::Greater)
        } else {
            self.suit.partial_cmp(&other.suit)
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_suit_order() {
        let c1 = Card::new(2, Suit::Diamonds);
        let c2 = Card::new(3, Suit::Diamonds);
        assert!(c1 < c2);
    }

    #[test]
    fn test_different_suit_order() {
        let c1 = Card::new(2, Suit::Diamonds);
        let c2 = Card::new(3, Suit::Clubs);
        assert!(c1 < c2);
    }

    #[test]
    fn test_same_rank_order() {
        let c1 = Card::new(2, Suit::Spades);
        let c2 = Card::new(2, Suit::Hearts);
        assert!(c1 < c2);
    }

    #[test]
    fn test_same_card() {
        let c1 = Card::new(2, Suit::Spades);
        let c2 = Card::new(2, Suit::Spades);
        assert_eq!(c1, c2);
    }
}