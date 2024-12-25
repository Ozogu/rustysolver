use crate::deck::Deck;
use crate::player_cards::PlayerCards;
use crate::hole_cards::HoleCards;

#[derive(Clone, Debug)]
pub struct Deal {
    pub cards: PlayerCards,
    pub deck: Deck,
}

impl Deal {
    pub fn new(cards: PlayerCards, deck: Deck) -> Self {
        Deal {
            cards,
            deck,
        }
    }

    pub fn new_default() -> Self {
        Deal {
            cards: PlayerCards::new(
                HoleCards::new_with_rank(0),
                HoleCards::new_with_rank(0),
            ),
            deck: Deck::new_empty(),
        }
    }
}