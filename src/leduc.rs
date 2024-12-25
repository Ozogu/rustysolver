use crate::deck::Deck;
use crate::game::Game;
use crate::history::History;
use crate::suit::Suit;
use crate::card::Card;
use crate::pot::Pot;
use crate::action::Action;

#[derive(Clone, Debug)]
pub struct Leduc {}

impl Leduc {
    pub fn new() -> Self {
        Leduc {}
    }
}

impl Game for Leduc {
    fn initial_pot(&self) -> Pot {
        Pot::new(1.0, 1.0)
    }

    fn deck(&self) -> crate::deck::Deck {
        Deck::new_from_cards(
            vec![
                Card::new(1, Suit::Diamonds),
                Card::new(1, Suit::Clubs),
                Card::new(2, Suit::Diamonds),
                Card::new(2, Suit::Clubs),
                Card::new(3, Suit::Diamonds),
                Card::new(3, Suit::Clubs),
            ]
        )   
    }

    fn num_streets(&self) -> u8 {
        2
    }

    fn num_hole_cards(&self) -> u8 {
        1
    }

    fn get_legal_actions(&self, history: &History) -> Vec<Action> {
        vec![]
    }

    fn get_legal_first_actions() -> Vec<Action> {
        vec![Action::Check, Action::Bet(50)]
    }
}