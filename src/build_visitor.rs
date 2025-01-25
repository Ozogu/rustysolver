use std::collections::HashMap;
use crate::node::Node;
use crate::visitor::Visitor;
use crate::info_state::InfoState;

pub struct BuilderVisitor {
    pub regrets: HashMap<InfoState, Vec<f64>>,
    pub strategy_sum: HashMap<InfoState, Vec<f64>>,
}

impl BuilderVisitor {
    pub fn new() -> Self {
        BuilderVisitor {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &Node) {
        let info_state = node.info_state().clone();
        self.regrets.entry(info_state.clone()).or_insert(node.zero_utils());
        self.strategy_sum.entry(info_state).or_insert(node.zero_utils());
    }
}

impl Visitor for BuilderVisitor {
    fn visit_node(&mut self, _: &Node) {}
    fn visit_terminal_node(&mut self, _: &Node) {}

    fn visit_street_completing_node(&mut self, node: &Node) {
        self.add_node(node);
    }

    fn visit_action_node(&mut self, node: &Node) {
        self.add_node(node);
    }
}