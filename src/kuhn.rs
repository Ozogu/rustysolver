use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use crate::action::Action;
use crate::info_state::InfoState;

#[derive(Clone, Debug)]
pub struct Kuhn {
    cards: Vec<usize>
}

impl Kuhn {
    pub fn new() -> Self {
        Kuhn {
            cards: vec![0, 1, 2],
        }
    }

    pub fn shuffled_cards(&self, rng: &mut StdRng) -> Vec<usize> {
        let mut cards = self.cards.clone();
        cards.shuffle(rng);
        cards
    }

    pub fn get_legal_actions(&self, info_state: &InfoState) -> Vec<Action> {
        // At root there will be no history
        let last = info_state.last().unwrap_or(&Action::Check);
        if last == &Action::Check {
            vec![Action::Check, Action::Bet(50)]
        } else { // last == Bet
            vec![Action::Call, Action::Fold]
        }
    }

    pub fn get_payoff(&self, info_state: &InfoState) -> f64 {
        let player = info_state.player();
        let opponent = player.opponent();
        let actions: Vec<Action> = info_state.history().to_vec();
        if actions == vec![Action::Check, Action::Check] {
            if self.cards[player.as_usize()] > self.cards[opponent.as_usize()] {
                1.0
            } else {
                -1.0
            }
        } else if actions == vec![Action::Bet(50), Action::Fold] || actions == vec![Action::Check, Action::Bet(50), Action::Fold] {
            1.0
        } else if actions == vec![Action::Bet(50), Action::Call] || actions == vec![Action::Check, Action::Bet(50), Action::Call] {
            if self.cards[player.as_usize()] > self.cards[opponent.as_usize()] {
                2.0
            } else {
                -2.0
            }
        } else {
            panic!("Invalid game state: history = {:?}, cards = {:?}", actions, self.cards);
        }
    }
}