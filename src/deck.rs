use crate::card::Card;
use crate::suit::Suit;
use rand::seq::SliceRandom;
use rand::rngs::StdRng;

#[derive(Clone, Debug, PartialEq)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = Vec::new();
        for rank in 2..15 {
            for suit in Suit::to_vec() {
                cards.push(Card::new(rank, suit));
            }
        }
        Deck { cards }
    }

    pub fn new_from_cards(cards: Vec<Card>) -> Self {
        Deck { cards }
    }

    pub fn new_empty() -> Self {
        Deck { cards: Vec::new() }
    }

    pub fn shuffle(&mut self, rng: &mut StdRng) {
        self.cards.shuffle(rng);
    }

    pub fn reverse(&mut self) {
        self.cards.reverse();
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }
}