use crate::info_state::InfoState;
use crate::visitor::Visitor;
use crate::node::Node;
use crate::action::Action;
use std::collections::HashMap;
use crate::game_tree::GameTree;
use crate::game::Game;

pub struct StatisticsVisitor<'a, G: Game + Clone> {
    stat_node: HashMap<InfoState, StatisticsNode>,
    tree: &'a GameTree<G>,
}

impl<'a, G: Game + Clone> StatisticsVisitor<'a, G> {
    pub fn new(tree: &'a GameTree<G>) -> Self {
        StatisticsVisitor {
            stat_node: HashMap::new(),
            tree
        }
    }

    pub fn node_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_node.get(info_state).unwrap();
        let updates = if stat_node.updates > 0 { stat_node.updates as f64 } else { 1.0 };

        stat_node.log();

        stat_node.util_sum / updates
    }

    pub fn node_br_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_node.get(info_state).unwrap();
        let updates = stat_node.updates as f64;

        stat_node.br_util / updates
    }

    pub fn node_exploitability(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_node.get(info_state).unwrap();
        let br_util = stat_node.br_util;
        let util = stat_node.util_sum;
        let updates = stat_node.updates as f64;

        ((br_util - util) / updates) / util.abs() * 100.0
    }
}

impl<'a, G: Game + Clone> Visitor for StatisticsVisitor<'a, G> {
    fn visit_root_node(&mut self, info_state: &InfoState, util: f64) {
        let stat_node = self.stat_node.entry(info_state.clone()).or_insert(
            StatisticsNode::new(0));

        stat_node.reach_prob_sum = 1.0;
        stat_node.util_sum += util;
        stat_node.updates += 1;
    }

    fn visit_terminal_node(&mut self, _: &Node) {}
    fn visit_street_completing_node(&mut self, _: &Node) {}

    fn visit_action_node(&mut self, node: &Node) {
        let stat_node = self.stat_node
            .entry(node.info_state()).or_insert(
                StatisticsNode::new(node.actions.len()));

        update_node(stat_node, node);
        // node.log();
        // stat_node.log();
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
    stat_node.reach_prob_sum += node.player_reach_prob() * node.opponent_reach_prob();
    stat_node.util_sum += node.util * node.player_reach_prob() * node.opponent_reach_prob();

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
    pub reach_prob_sum: f64,
    pub updates: u32,
}

impl StatisticsNode {
    fn new(actions: usize) -> Self {
        StatisticsNode {
            util_sum: 0.0,
            action_util_sums: vec![0.0; actions],
            br_util: 0.0,
            best_response: Action::None,
            reach_prob_sum: 0.0,
            updates: 0,
        }
    }

    fn log(&self) {
        println!("Util sum: {:.4}", self.util_sum);
        println!("Action util sums: {:?}", self.action_util_sums);
        println!("BR Util: {:.4}", self.br_util);
        println!("Best response: {:?}", self.best_response);
        println!("Reach prob sum: {:.4}", self.reach_prob_sum);
        println!("Updates: {}", self.updates);
        println!("_________________");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history_node::HistoryNode;
    use crate::hole_cards::HoleCards;
    use crate::info_state::InfoState;
    use crate::kuhn::Kuhn;
    use crate::ideal_kuhn_builder_visitor::IdealKuhnBuilderVisitor;
    use crate::player::Player;
    use crate::tree_walker::TreeWalker;
    use crate::bet::Bet;
    use crate::history::History;

    #[test]
    fn test_root() {
        let tree: GameTree<Kuhn> = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        TreeWalker::walk_tree(&Kuhn::new(), &mut statistics_visitor);

        let info_state = InfoState::new_empty();
        assert!((statistics_visitor.node_util(&info_state) - (-1.0/18.0)).abs() < 1e-6);
    }

    #[test]
    fn test_oop_1() {
        // let tree = IdealKuhnBuilderVisitor::new().tree;
        // let mut statistics_visitor = StatisticsVisitor::new(&tree);
        // TreeWalker::walk_tree(&Kuhn::new(), &mut statistics_visitor);

        // let info_state = InfoState::new(Player::OOP, HoleCards::new_with_rank(1), History::new());
        // assert_eq!(statistics_visitor.node_util(&info_state), -1.0/18.0);
    }

    #[test]
    fn test_oop_1_x_b() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        TreeWalker::walk_tree(&Kuhn::new(), &mut statistics_visitor);

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_rank(1), History::new_from_vec(
            vec![HistoryNode::Action(Action::Check), HistoryNode::Action(Action::Bet(Bet::P(50)))]));

        assert_eq!(statistics_visitor.node_util(&info_state), -0.5);
    }
}