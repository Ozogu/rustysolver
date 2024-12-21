use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use crate::action::Action;
use crate::history::History;
use crate::player::Player;
use crate::info_state::InfoState;

#[derive(Clone, Debug)]
pub struct Kuhn {
    cards: Vec<usize>,
    info_state: InfoState,
    player: Player,
}

impl Kuhn {
    pub fn new(rng: &mut StdRng) -> Self {
        let mut cards = vec![0, 1, 2];
        let mut rng = rng;
        cards.shuffle(&mut rng);
        Kuhn {
            cards,
            info_state: InfoState::new_empty(),
            player: Player::IP,
        }
    }

    pub fn get_legal_actions(&self) -> Vec<Action> {
        // At root there will be no history
        let last = self.info_state.last().unwrap_or(&Action::Check);
        if last == &Action::Check {
            vec![Action::Check, Action::Bet(50)]
        } else { // last == Bet
            vec![Action::Call, Action::Fold]
        }
    }

    pub fn get_payoff(&self, player: Player) -> f64 {
        let opponent = player.opponent();
        let actions: Vec<Action> = self.info_state.history().to_vec();
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
            panic!("Invalid game state: history = {:?}, cards = {:?}", self.info_state, self.cards)
        }
    }

    pub fn next_state(&self, action: Action) -> Kuhn {
        let mut next_state = self.clone();
        next_state.info_state.push(action);
        next_state.player = self.player.opponent();
        next_state
    }

    pub fn get_history(&self) -> History {
        self.info_state.history()
    }

    pub fn get_cards(&self) -> Vec<usize> {
        self.cards.clone()
    }

    pub fn get_player_cards(&self) -> usize {
        self.cards[self.player.as_usize()]
    }
}