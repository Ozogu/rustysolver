use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Bet {
    C(u32),
    P(u32),
}

impl fmt::Display for Bet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Bet::C(chip) => write!(f, "{}c", chip),
            Bet::P(percentage) => write!(f, "{:.2}", percentage),
        }
    }
}