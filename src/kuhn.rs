use rand::seq::SliceRandom;
use rand::rngs::StdRng;

#[derive(Clone, Debug)]
pub struct KuhnPoker {
    pub cards: Vec<usize>,
    pub history: Vec<String>,
    player: usize,
}

impl KuhnPoker {
    pub fn new(rng: &mut StdRng) -> Self {
        let mut cards = vec![0, 1, 2];
        let mut rng = rng;
        cards.shuffle(&mut rng);
        KuhnPoker {
            cards,
            history: Vec::new(),
            player: 0
        }
    }

    pub fn get_legal_actions(&self) -> Vec<String> {
        vec!["p".to_string(), "b".to_string()]
    }

    pub fn is_terminal(&self) -> bool {
        let history_str = self.history.join("");
        history_str == "pbp" ||
        history_str == "pbb" ||
        history_str == "pp" ||
        history_str == "bp" ||
        history_str == "bb"
    }

    pub fn get_payoff(&self, player: usize) -> f64 {
        let opponent = 1 - player;
        let history_str = self.history.join("");
        if history_str == "pp" {
            if self.cards[player] > self.cards[opponent] {
                1.0
            } else {
                -1.0
            }
        } else if history_str == "bp" || history_str == "pbp" {
            1.0
        } else if history_str == "bb" || history_str == "pbb"{
            if self.cards[player] > self.cards[opponent] {
                2.0
            } else {
                -2.0
            }
        } else {
            panic!("Invalid game state: history = {:?}, cards = {:?}", self.history, self.cards)
        }
    }

    pub fn next_state(&self, action: &str) -> KuhnPoker {
        let mut next_state = self.clone();
        next_state.history.push(action.to_string());
        next_state.player = 1 - self.player;
        next_state
    }

    pub fn get_history(&self) -> Vec<String> {
        self.history.clone()
    }

    pub fn get_cards(&self) -> Vec<usize> {
        self.cards.clone()
    }

    pub fn print_deck(&self) {
        println!("Deck: {:?}", self.cards);
    }
}