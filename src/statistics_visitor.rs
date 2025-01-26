use crate::history::History;
use crate::visitor::Visitor;
use crate::node::Node;
use crate::action::Action;
use std::collections::HashMap;
use crate::game_tree::GameTree;
use crate::game::Game;

pub struct StatisticsVisitor<'a, G: Game + Clone> {
    history_stats: HashMap<History, StatisticsNode>,
    tree: &'a GameTree<G>,
}

impl<'a, G: Game + Clone> StatisticsVisitor<'a, G> {
    pub fn new(tree: &'a GameTree<G>) -> Self {
        StatisticsVisitor {
            history_stats: HashMap::new(),
            tree
        }
    }

    pub fn node_util(&self, history: &History) -> f64 {
        let stat_node = self.history_stats.get(history).unwrap();
        let updates = stat_node.updates as f64;

        stat_node.util_sum / updates
    }

    pub fn node_br_util(&self, history: &History) -> f64 {
        let stat_node = self.history_stats.get(history).unwrap();
        let updates = stat_node.updates as f64;

        stat_node.br_util / updates
    }

    pub fn node_exploitability(&self, history: &History) -> f64 {
        let stat_node = self.history_stats.get(history).unwrap();
        let br_util = stat_node.br_util;
        let util = stat_node.util_sum;
        let updates = stat_node.updates as f64;

        ((br_util - util) / updates) / util.abs() * 100.0
    }
}

impl<'a, G: Game + Clone> Visitor for StatisticsVisitor<'a, G> {
    fn visit_node(&mut self, _: &Node) {}
    fn visit_terminal_node(&mut self, _: &Node) {}
    fn visit_street_completing_node(&mut self, _: &Node) {}

    fn visit_action_node(&mut self, node: &Node) {
        let history_node = self.history_stats
            .entry(node.history.clone()).or_insert(
                StatisticsNode::new(node.actions.len()));

        update_node(history_node, node);
    }

    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        self.tree.average_strategy(&node.info_state())
    }
}

fn arg_max(vec: &Vec<f64>) -> usize {
    let mut max = vec[0];
    let mut max_index = 0;

    for i in 1..vec.len() {
        if vec[i] > max {
            max = vec[i];
            max_index = i;
        }
    }

    max_index
}

fn update_node(stat_node: &mut StatisticsNode, node: &Node) {

    stat_node.updates += 1;
    stat_node.util_sum += node.util;

    for i in 0..stat_node.action_util_sums.len() {
        stat_node.action_util_sums[i] += node.action_utils[i];
    }

    let i = arg_max(&stat_node.action_util_sums);
    stat_node.br_util = stat_node.action_util_sums[i];
    stat_node.best_response = node.actions[i].clone();
}

#[derive(Clone, Debug)]
struct StatisticsNode {
    pub util_sum: f64,
    pub action_util_sums: Vec<f64>,
    pub br_util: f64,
    pub best_response: Action,
    pub updates: u32,
}

impl StatisticsNode {
    fn new(actions: usize) -> Self {
        StatisticsNode {
            util_sum: 0.0,
            action_util_sums: vec![0.0; actions],
            br_util: 0.0,
            best_response: Action::None,
            updates: 0,
        }
    }

    fn log(&self) {
        println!("Util sum: {:.4}", self.util_sum);
        println!("Action util sums: {:?}", self.action_util_sums);
        println!("BR Util: {:.4}", self.br_util);
        println!("Best response: {:?}", self.best_response);
        println!("Updates: {}", self.updates);
        println!("_________________");
    }
}