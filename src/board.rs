use crate::card::Card;

#[derive(Debug, Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cards: vec![],
        }
    }

    pub fn from_vec(cards: Vec<Card>) -> Self {
        Board {
            cards,
        }
    }

    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn to_string(&self) -> String {
        self.cards.iter().map(|c| c.to_string()).collect::<Vec<String>>().join("")
    }

    pub fn to_vec(&self) -> Vec<Card> {
        self.cards.clone()
    }
}