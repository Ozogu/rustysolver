use crate::node::Node;
use crate::info_state::InfoState;

pub trait Visitor {
    fn visit_root_node(&mut self, info_state: &InfoState, util: f64);
    fn visit_terminal_node(&mut self, node: &Node);
    fn visit_street_completing_node(&mut self, node: &Node);
    fn visit_action_node(&mut self, node: &Node);
    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        vec![1.0; node.actions.len()]
    }
}