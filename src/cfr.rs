use std::collections::HashMap;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::info_state::InfoState;
use crate::kuhn::Kuhn;
use crate::node::Node;

pub struct CFR {
    game: Kuhn,
    regrets: HashMap<String, Vec<f64>>,
    strategy_sum: HashMap<String, Vec<f64>>,
    rng: StdRng,
}

impl CFR {
    pub fn new(game: Kuhn) -> Self {
        CFR {
            game,
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            rng: StdRng::seed_from_u64(0),
        }
    }

    pub fn get_strategy(&mut self, info_state: &InfoState, realization_weight: f64) -> Vec<f64> {
        let info_set = info_state.to_string();
        let actions = self.regrets.entry(info_set.clone()).or_insert(vec![0.0; 2]);
        let mut strategy = vec![0.0; actions.len()];
        let mut normalizing_sum = 0.0;

        for (i, regret) in actions.iter().enumerate() {
            strategy[i] = if *regret > 0.0 { *regret } else { 0.0 };
            normalizing_sum += strategy[i];
        }

        for i in 0..strategy.len() {
            if normalizing_sum > 0.0 {
                strategy[i] /= normalizing_sum;
            } else {
                strategy[i] = 1.0 / strategy.len() as f64;
            }
            self.strategy_sum.entry(info_set.clone()).or_insert(vec![0.0; 2])[i] += realization_weight * strategy[i];
        }

        strategy
    }

    pub fn cfr(&mut self, node: Node) -> f64 {
        if node.is_terminal() {
            return self.game.get_payoff(&node);
        }
        
        let info_state = &node.info_state;
        let actions = self.game.get_legal_actions(info_state);
        let strategy = self.get_strategy(info_state, node.player_reach_prob());

        let mut action_util = vec![0.0; actions.len()];
        let mut node_util = 0.0;

        for (i, action) in actions.iter().enumerate() {
            let next_node = node.next_node(action.clone(), strategy[i]);
            action_util[i] = -self.cfr(next_node);
            node_util += strategy[i] * action_util[i];
        }

        for (i, _) in actions.iter().enumerate() {
            let regret = action_util[i] - node_util;
            self.regrets.entry(info_state.to_string()).or_insert(vec![0.0; actions.len()])[i] += node.opponent_reach_prob() * regret;
        }

        node_util
    }

    pub fn train(&mut self, iterations: usize) -> f64 {
        let mut ev = 0.0;
        for _ in 0..iterations {
            let cards = self.game.shuffled_cards(&mut self.rng);
            ev += self.cfr(Node::new(&cards));
        }

        return ev / iterations as f64;
    }

    pub fn get_average_strategy(&self, info_set: &str) -> Option<Vec<f64>> {
        if !self.strategy_sum.contains_key(info_set) {
            return None;
        }

        let strategy_sum = self.strategy_sum.get(info_set).unwrap();
        let mut avg_strategy = vec![0.0; strategy_sum.len()];
        let mut normalizing_sum = 0.0;

        for value in strategy_sum.iter() {
            normalizing_sum += *value;
        }

        for i in 0..avg_strategy.len() {
            if normalizing_sum > 0.0 {
                avg_strategy[i] = strategy_sum[i] / normalizing_sum;
            } else {
                avg_strategy[i] = 1.0 / avg_strategy.len() as f64;
            }
        }

        Some(avg_strategy)
    }
}
