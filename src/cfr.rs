use std::collections::HashMap;
use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::info_state::InfoState;
use crate::game::Game;
use crate::node::Node;

pub struct CFR<G: Game> {
    game: G,
    regrets: HashMap<InfoState, Vec<f64>>,
    strategy_sum: HashMap<InfoState, Vec<f64>>,
    rng: StdRng,
}

impl<G: Game> CFR<G> {
    pub fn new(game: G) -> Self {
        CFR {
            game,
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            rng: StdRng::seed_from_u64(0),
        }
    }

    pub fn train(&mut self, iterations: usize) -> f64 {
        let mut ev = 0.0;
        for _ in 0..iterations {
            let (cards, _) = self.game.deal(&mut self.rng);
            ev += self.cfr(Node::new(&self.game, cards));
        }

        return ev / iterations as f64;
    }

    pub fn print_strategy(&mut self) {
        for (info_state, _) in &self.regrets {
            let avg_strategy = self.get_average_strategy(info_state).unwrap();
            println!("{:}: {:.2?}", info_state, avg_strategy);
        }
    }

    fn cfr(&mut self, node: Node) -> f64 {
        if node.is_terminal() {
            let won = self.game.player_wins(&node);
            let win_amount = node.pot.payoff(node.player(), won);

            return win_amount;
        }
        
        self.create_node_entry(&node);
        let info_state = &node.info_state;
        let strategy = self.get_strategy(&node);

        let mut action_util = node.zero_utils();
        let mut node_util = 0.0;

        for i in 0..node.actions.len() {
            let next_node = node.next_node(&self.game, node.actions[i].clone(), strategy[i]);
            action_util[i] = -self.cfr(next_node);
            node_util += strategy[i] * action_util[i];
        }

        for i in 0..node.actions.len() {
            let regret = action_util[i] - node_util;
            self.regrets.get_mut(info_state).unwrap()[i] += node.opponent_reach_prob() * regret;
        }

        node_util
    }

    fn get_strategy(&mut self, node: &Node) -> Vec<f64> {
        let actions = self.regrets.get(&node.info_state).unwrap();
        let mut strategy: Vec<f64> = node.zero_utils();
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
            self.strategy_sum.get_mut(&node.info_state).unwrap()[i] += node.player_reach_prob() * strategy[i];
        }

        strategy
    }

    fn get_average_strategy(&self, info_state: &InfoState) -> Option<Vec<f64>> {
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

    fn create_node_entry(&mut self, node: &Node) {
        self.regrets.entry(node.info_state.clone()).or_insert(node.zero_utils());
        self.strategy_sum.entry(node.info_state.clone()).or_insert(node.zero_utils());
    }
}
