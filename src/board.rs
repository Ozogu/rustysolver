use crate::card::Card;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    pub cardss: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cardss: vec![],
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cardss.push(card);
    }
}