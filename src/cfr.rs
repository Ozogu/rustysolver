use std::collections::HashMap;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::info_state::InfoState;
use crate::kuhn::Kuhn;
use crate::node::Node;

pub struct CFR {
    game: Kuhn,
    regrets: HashMap<InfoState, Vec<f64>>,
    strategy_sum: HashMap<InfoState, Vec<f64>>,
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

    pub fn get_strategy(&mut self, node: &Node) -> Vec<f64> {
        let actions = self.regrets.entry(node.info_state.clone()).or_insert(vec![0.0; 2]);
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
            self.strategy_sum.entry(node.info_state.clone()).or_insert(vec![0.0; 2])[i] += node.player_reach_prob() * strategy[i];
        }

        strategy
    }

    pub fn cfr(&mut self, node: Node) -> f64 {
        if node.is_terminal() {
            return self.game.get_payoff(&node);
        }
        
        let info_state = &node.info_state;
        let actions = self.game.get_legal_actions(info_state);
        let strategy = self.get_strategy(&node);

        let mut action_util = vec![0.0; actions.len()];
        let mut node_util = 0.0;

        for i in 0..actions.len() {
            let next_node = node.next_node(actions[i].clone(), strategy[i]);
            action_util[i] = -self.cfr(next_node);
            node_util += strategy[i] * action_util[i];
        }

        for i in 0..actions.len() {
            let regret = action_util[i] - node_util;
            self.regrets.entry(info_state.clone()).or_insert(vec![0.0; actions.len()])[i] += node.opponent_reach_prob() * regret;
        }

        node_util
    }

    pub fn train(&mut self, iterations: usize) -> f64 {
        let mut ev = 0.0;
        for _ in 0..iterations {
            let (ip_cards, oop_cards, _) = self.game.deal(&mut self.rng);
            ev += self.cfr(Node::new(ip_cards, oop_cards));
        }

        return ev / iterations as f64;
    }

    pub fn get_average_strategy(&self, info_state: &InfoState) -> Option<Vec<f64>> {
        if !self.strategy_sum.contains_key(info_state) {
            return None;
        }

        let strategy_sum = self.strategy_sum.get(info_state).unwrap();
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
