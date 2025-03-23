use crate::game::Game;
use crate::pot::Pot;
use crate::postflop_holdem_config::PostflopHoldemConfig;
use crate::action::Action;
use crate::deal::Deal;
use crate::history::History;
use crate::deck::Deck;
use rand::rngs::StdRng;

pub struct PostflopHoldem {
    config: PostflopHoldemConfig,
}

impl PostflopHoldem {
    pub fn new(config: PostflopHoldemConfig) -> Self {
        PostflopHoldem {
            config,
        }
    }
}

impl Game for PostflopHoldem {
    fn initial_pot(&self) -> Pot {
        // TODO: From config
        Pot::new(1.0, 1.0)
    }

    fn deck(&self) -> Deck {
        Deck::new()
    }

    fn num_streets(&self) -> u8 {
        3
    }

    fn legal_actions(&self, history: &History) -> Vec<Action> {
        // TODO: From config
        vec![]
    }

    fn legal_first_actions(&self) -> Vec<Action> {
        // TODO: From config
        vec![]
    }

    fn deal(&self, _rng: &mut StdRng) -> Deal {
        Deal::new_default()
    }
}

