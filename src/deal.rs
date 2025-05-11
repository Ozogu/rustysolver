use crate::deck::Deck;
use crate::player_cards::PlayerCards;
use crate::hole_cards::HoleCards;
use crate::history::History;

#[derive(Clone, Debug)]
pub struct Deal {
    pub cards: PlayerCards,
    // IP & OOP
    pub weights: (f64, f64),
    pub deck: Deck,
    pub history: History,
}

impl Deal {
    pub fn new(cards: PlayerCards, deck: Deck, weights: (f64, f64), history: History) -> Self {
        Deal {
            cards,
            weights,
            deck,
            history
        }
    }

    pub fn new_default() -> Self {
        Deal {
            cards: PlayerCards::new(
                HoleCards::new_with_ranks(1, 1),
                HoleCards::new_with_ranks(2, 1),
            ),
            weights: (1.0, 1.0),
            deck: Deck::new_empty(),
            history: History::new(),
        }
    }
}