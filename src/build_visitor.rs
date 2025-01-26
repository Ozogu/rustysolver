use crate::node::Node;
use crate::visitor::Visitor;
use crate::game::Game;
use crate::game_tree::GameTree;

pub struct BuilderVisitor<'a, G: Game + Clone> {
    pub tree: &'a mut GameTree<G>,
}

impl<'a, G: Game + Clone> BuilderVisitor<'a, G> {
    pub fn new(tree: &'a mut GameTree<G>) -> Self {
        BuilderVisitor {
            tree
        }
    }

    fn add_node(&mut self, node: &Node) {
        let info_state = node.info_state().clone();
        self.tree.regrets.entry(info_state.clone()).or_insert(node.zero_utils());
        self.tree.strategy_sum.entry(info_state).or_insert(node.zero_utils());
    }
}

impl<'a, G: Game + Clone> Visitor for BuilderVisitor<'a, G> {
    fn visit_street_completing_node(&mut self, node: &Node) {
        self.add_node(node);
    }

    fn visit_action_node(&mut self, node: &Node) {
        self.add_node(node);
    }
}