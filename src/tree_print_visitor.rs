use crate::game_tree::GameTree;
use crate::info_state::InfoState;
use crate::visitor::Visitor;
use crate::game::Game;
use crate::node::Node;

pub struct TreePrintVisitor<'a, G: Game + Clone> {
    lines: Vec<(InfoState, String)>,
    tree: &'a GameTree<G>,
}

impl<'a, G: Game + Clone> TreePrintVisitor<'a, G> {
    pub fn new(tree: &'a GameTree<G>) -> Self {
        TreePrintVisitor {
            lines: Vec::new(),
            tree,
        }
    }

    pub fn print(&mut self) {
        self.lines.sort_by_key(|(info_state, _)| (
            info_state.player,
            info_state.hole_cards.clone(),
            info_state.history.to_string().len(),
            info_state.history.clone(),
        ));

        for (_, line) in &self.lines {
            println!("{:}", line);
        }
    }
}

impl<'a, G: Game + Clone> Visitor for TreePrintVisitor<'a, G> {
    fn visit_node(&mut self, node: &Node) {
        let actions = &node.actions;
        let strategy = self.tree.average_strategy(&node.info_state());

        let mut line = String::new();
        line.push_str("[");
        for (action, prob) in actions.iter().zip(strategy.iter()) {
            line.push_str(&format!("{:}:{:.2?}, ", action, prob));
        }
        line.pop();
        line.pop();
        line.push_str("]");

        self.lines.push((node.info_state().clone(), line));
    }

    fn visit_terminal_node(&mut self, _: &Node) {}
    fn visit_street_completing_node(&mut self, _: &Node) {}
    fn visit_action_node(&mut self, _: &Node) {}
}