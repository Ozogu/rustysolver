use crate::node::Node;

pub trait Visitor {
    fn visit_terminal_node(&mut self, node: &Node);
    fn visit_street_completing_node(&mut self, node: &Node);
    fn visit_action_node(&mut self, node: &Node);
}