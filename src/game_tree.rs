use std::collections::HashMap;
use crate::info_state::InfoState;
use crate::game::Game;

pub struct GameTree<G: Game> {
    regrets: HashMap<InfoState, Vec<f64>>,
    strategy_sum: HashMap<InfoState, Vec<f64>>,
    game: G,
}

impl<G: Game> GameTree<G> {
    pub fn new(game: G) -> Self {
        GameTree {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            game,
        }
    }

    pub fn build(&mut self) {
    }
}