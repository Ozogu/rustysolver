use rand::seq::SliceRandom;
use rand::rngs::StdRng;
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
            info_state: InfoState::new(),
            player: Player::IP,
        }
    }

    pub fn get_legal_actions(&self) -> Vec<String> {
        vec!["p".to_string(), "b".to_string()]
    }

    pub fn is_terminal(&self) -> bool {
        let info_state_str = self.info_state.to_string();
        info_state_str == "pbp" ||
        info_state_str == "pbb" ||
        info_state_str == "pp" ||
        info_state_str == "bp" ||
        info_state_str == "bb"
    }

    pub fn get_payoff(&self, player: Player) -> f64 {
        let opponent = if player == Player::IP { Player::OOP } else { Player::IP };
        let info_state_str = self.info_state.to_string();
        if info_state_str == "pp" {
            if self.cards[player as usize] > self.cards[opponent as usize] {
                1.0
            } else {
                -1.0
            }
        } else if info_state_str == "bp" || info_state_str == "pbp" {
            1.0
        } else if info_state_str == "bb" || info_state_str == "pbb"{
            if self.cards[player as usize] > self.cards[opponent as usize] {
                2.0
            } else {
                -2.0
            }
        } else {
            panic!("Invalid game state: history = {:?}, cards = {:?}", self.info_state, self.cards)
        }
    }

    pub fn next_state(&self, action: &str) -> Kuhn {
        let mut next_state = self.clone();
        next_state.info_state.push(action.to_string());
        next_state.player = if self.player == Player::IP { Player::OOP } else { Player::IP };
        next_state
    }

    pub fn get_history(&self) -> Vec<String> {
        self.info_state.clone().into_vec()
    }

    pub fn get_cards(&self) -> Vec<usize> {
        self.cards.clone()
    }

    pub fn get_player_cards(&self) -> usize {
        self.cards[self.player as usize]
    }

    pub fn get_info_state_str(&self) -> Vec<String> {
        self.info_state.clone().into_vec()
    }
}