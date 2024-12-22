#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Hash)]
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

    pub fn from_string(s: &str) -> Self {
        match s {
            "♠" => Suit::Spades,
            "♥" => Suit::Hearts,
            "♦" => Suit::Diamonds,
            "♣" => Suit::Clubs,
            _ => panic!("Invalid suit: {}", s),
        }
    }
}