use crate::card::Card;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cards: vec![],
        }
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }
}