use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Suit {
    Hearts,
    Spades,
    Diamonds,
    Clubs,
    Suited,
    Offsuit,
}

impl Suit {
    pub fn to_vec() -> Vec<Suit> {
        vec![Suit::Spades, Suit::Hearts, Suit::Diamonds, Suit::Clubs]
    }

    pub fn to_string(&self) -> String {
        match self {
            Suit::Hearts => "♥".to_string(),
            Suit::Spades => "♠".to_string(),
            Suit::Diamonds => "♦".to_string(),
            Suit::Clubs => "♣".to_string(),
            Suit::Suited => "u".to_string(),
            Suit::Offsuit => "o".to_string(),
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s {
            "♥" => Suit::Hearts,
            "♠" => Suit::Spades,
            "♦" => Suit::Diamonds,
            "♣" => Suit::Clubs,
            "s" => Suit::Spades,
            "h" => Suit::Hearts,
            "d" => Suit::Diamonds,
            "c" => Suit::Clubs,
            "u" => Suit::Suited,
            "o" => Suit::Offsuit,
            _ => panic!("Invalid suit: {}", s),
        }
    }

    pub fn to_usize(&self) -> usize {
        match self {
            Suit::Hearts => 0,
            Suit::Spades => 1,
            Suit::Diamonds => 2,
            Suit::Clubs => 3,
            Suit::Suited => 4,
            Suit::Offsuit => 5,
        }
    }

    pub fn from_u8(s: u8) -> Self {
        match s {
            0 => Suit::Spades,
            1 => Suit::Hearts,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            4 => Suit::Suited,
            5 => Suit::Offsuit,
            _ => panic!("Invalid suit: {}", s),
        }
    }
}

impl PartialOrd for Suit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.to_usize().cmp(&self.to_usize()))
    }
}

impl Ord for Suit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}