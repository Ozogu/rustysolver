use std::collections::HashMap;
use crate::history::History;
use crate::node::Node;

pub struct Statistics {
    nodes: HashMap<History, StatisticsNode>,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            nodes: HashMap::new(),
        }
    }

    pub fn update_node_utils(&mut self, node: &Node, util: f64, action_utils: Vec<f64>) {
        let history = node.history.clone();
        let node = self.nodes.entry(history).or_insert(StatisticsNode {
            util: 0.0,
            action_utils: node.zero_utils(),
            updates: 0,
        });

        node.updates += 1;
        node.util += util;
        for i in 0..node.action_utils.len() {
            node.action_utils[i] += action_utils[i];
        }
    }
    
    pub fn finalize(&mut self) {
        for (_, node) in self.nodes.iter_mut() {
            node.util /= node.updates as f64;
            for i in 0..node.action_utils.len() {
                node.action_utils[i] /= node.updates as f64;
            }
        }
    }

    pub fn get_node_util(&self, history: &History) -> f64 {
        self.nodes.get(history).unwrap().util
    }

    pub fn get_action_utils(&self, history: &History) -> Vec<f64> {
        self.nodes.get(history).unwrap().action_utils.clone()
    }
}

struct StatisticsNode {
    pub util: f64,
    pub action_utils: Vec<f64>,
    pub updates: u32,
}