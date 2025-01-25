use crate::visitor::Visitor;
use crate::node::Node;
use crate::info_state::InfoState;
use std::collections::HashMap;

pub struct CfrVisitor<'a> {
    regrets: &'a mut HashMap<InfoState, Vec<f64>>,
    strategy_sum: &'a mut HashMap<InfoState, Vec<f64>>,
}

impl<'a> CfrVisitor<'a> {
    pub fn new(
                regrets: &'a mut HashMap<InfoState, Vec<f64>>,
                strategy_sum: &'a mut HashMap<InfoState, Vec<f64>>) -> Self {
        CfrVisitor {
            regrets,
            strategy_sum,
        }
    }
}

impl<'a> Visitor for CfrVisitor<'a> {
    fn visit_terminal_node(&mut self, _: &Node) {
        return;
    }

    fn visit_street_completing_node(&mut self, _: &Node) {
        return;
    }

    fn visit_action_node(&mut self, node: &Node) {
        for i in 0..node.action_probs.len() {
            self.strategy_sum.get_mut(&node.info_state()).unwrap()[i] +=
                node.player_reach_prob() * node.action_probs[i];
        }

        for i in 0..node.actions.len() {
            let regret = node.action_utils[i] - node.util;
            self.regrets.get_mut(&node.info_state()).unwrap()[i] +=
                node.opponent_reach_prob() * regret;
        }
    }

    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        let regrets = self.regrets.get(&node.info_state()).unwrap();
        let mut strategy: Vec<f64> = node.zero_utils();
        let mut normalizing_sum = 0.0;

        for (i, regret) in regrets.iter().enumerate() {
            strategy[i] = if *regret > 0.0 { *regret } else { 0.0 };
            normalizing_sum += strategy[i];
        }

        for i in 0..strategy.len() {
            strategy[i] = if normalizing_sum > 0.0 {
                strategy[i] / normalizing_sum
            } else {
                1.0 / strategy.len() as f64
            };
        }

        strategy
    }
}