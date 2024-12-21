use std::collections::HashMap;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::kuhn::KuhnPoker;

pub struct CFR {
    regrets: HashMap<String, Vec<f64>>,
    strategy_sum: HashMap<String, Vec<f64>>,
    rng: StdRng,
}

impl CFR {
    pub fn new() -> Self {
        CFR {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            rng: StdRng::seed_from_u64(0),
        }
    }

    pub fn get_strategy(&mut self, info_set: &str, realization_weight: f64) -> Vec<f64> {
        let actions = self.regrets.entry(info_set.to_string()).or_insert(vec![0.0; 2]);
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
            self.strategy_sum.entry(info_set.to_string()).or_insert(vec![0.0; 2])[i] += realization_weight * strategy[i];
        }

        strategy
    }

    pub fn cfr(&mut self, state: &KuhnPoker, player: usize, p0: f64, p1: f64, depth: usize) -> f64 {
        if state.is_terminal() || depth > 10 {
            return state.get_payoff(player);
        }

        let info_set = format!("{}{}", state.cards[player], state.history.join(""));
        let actions = state.get_legal_actions();
        let strategy = self.get_strategy(&info_set, if player == 0 { p0 } else { p1 });

        let mut util = vec![0.0; actions.len()];
        let mut node_util = 0.0;

        for (i, action) in actions.iter().enumerate() {
            let next_state = state.next_state(action);
            util[i] = if player == 0 {
                -self.cfr(&next_state, 1, p0 * strategy[i], p1, depth + 1)
            } else {
                -self.cfr(&next_state, 0, p0, p1 * strategy[i], depth + 1)
            };
            node_util += strategy[i] * util[i];
        }

        for (i, _) in actions.iter().enumerate() {
            let regret = util[i] - node_util;
            self.regrets.entry(info_set.clone()).or_insert(vec![0.0; actions.len()])[i] += if player == 0 { p1 } else { p0 } * regret;
        }

        node_util
    }

    pub fn train(&mut self, iterations: usize) -> f64 {
        let mut ev = 0.0;
        for _ in 0..iterations {
            let state = KuhnPoker::new(&mut self.rng);
            ev += self.cfr(&state, 0, 1.0, 1.0, 0);
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
