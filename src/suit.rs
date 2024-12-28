use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash, Ord)]
pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl Suit {
    pub fn to_vec() -> Vec<Suit> {
        vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
    }

    pub fn to_string(&self) -> String {
        match self {
            Suit::Spades => "♠".to_string(),
            Suit::Hearts => "♥".to_string(),
            Suit::Diamonds => "♦".to_string(),
            Suit::Clubs => "♣".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "♠" => Suit::Spades,
            "♥" => Suit::Hearts,
            "♦" => Suit::Diamonds,
            "♣" => Suit::Clubs,
            "s" => Suit::Spades,
            "h" => Suit::Hearts,
            "d" => Suit::Diamonds,
            "c" => Suit::Clubs,
            _ => panic!("Invalid suit: {}", s),
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Suit::Spades => 0,
            Suit::Hearts => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3,
        }
    }

    pub fn from_u8(s: u8) -> Self {
        match s {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            _ => panic!("Invalid suit: {}", s),
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}