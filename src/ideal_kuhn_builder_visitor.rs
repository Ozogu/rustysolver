use crate::node::Node;
use crate::visitor::Visitor;
use crate::game_tree::GameTree;
use crate::kuhn::Kuhn;
use crate::tree_walker::TreeWalker;
use crate::info_state::InfoState;

pub struct IdealKuhnBuilderVisitor {
    pub tree: GameTree<Kuhn>,
    pub a: f64,
}

impl IdealKuhnBuilderVisitor {
    pub fn new() -> Self {
        let mut visitor = IdealKuhnBuilderVisitor {
            tree: GameTree::new(Kuhn::new()),
            a: 0.5,
        };

        TreeWalker::walk_tree(&Kuhn::new(), &mut visitor);

        visitor
    }

    fn add_node(&mut self, node: &Node) {
        let info_state  = node.info_state().clone();
        let a = self.a;

        let strategy = match info_state.to_string().as_str() {
            "OOP1♦1♦" => vec![1.0-a, a],
            "OOP2♦2♦" => vec![1.0, 0.0],
            "OOP3♦3♦" => vec![1.0-3.0*a, 3.0*a],
            "OOP1♦1♦XB50" => vec![1.0, 0.0],
            "OOP2♦2♦XB50" => vec![2.0/3.0-a, a+1.0/3.0],
            "OOP3♦3♦XB50" => vec![0.0, 1.0],

            "IP1♦1♦X" => vec![2.0/3.0, 1.0/3.0],
            "IP2♦2♦X" => vec![1.0, 0.0],
            "IP3♦3♦X" => vec![0.0, 1.0],
            "IP1♦1♦B50" => vec![1.0, 0.0],
            "IP2♦2♦B50" => vec![2.0/3.0, 1.0/3.0],
            "IP3♦3♦B50" => vec![0.0, 1.0],

            _ => panic!("Unknown info state: {:}", info_state)
        };

        self.tree.strategy_sum.insert(info_state, strategy);
    }
}

impl Visitor for IdealKuhnBuilderVisitor {
    fn visit_root_node(&mut self, _: &InfoState, _: f64) {}
    fn visit_terminal_node(&mut self, _: &Node) {}
    fn visit_street_completing_node(&mut self, _: &Node) {}

    fn visit_action_node(&mut self, node: &Node) {
        self.add_node(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history::History;
    use crate::hole_cards::HoleCards;
    use crate::info_state::InfoState;

    #[test]
    fn test_build_ideal_kuhn_tree() {
        let visitor = IdealKuhnBuilderVisitor::new();
        let a = visitor.a;

        let mut info_state = InfoState::new(crate::player::Player::OOP, HoleCards::new_with_rank(1), History::new());
        assert_eq!(visitor.tree.average_strategy(&info_state), vec![1.0-a, a]);

        info_state.hole_cards = HoleCards::new_with_rank(2);
        assert_eq!(visitor.tree.average_strategy(&info_state), vec![1.0, 0.0]);

        info_state.hole_cards = HoleCards::new_with_rank(3);
        assert_eq!(visitor.tree.average_strategy(&info_state), vec![1.0-3.0*a, 3.0*a]);
    }
}