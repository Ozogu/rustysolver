use crate::info_state::InfoState;
use crate::visitor::Visitor;
use crate::node::Node;
use crate::action::Action;
use std::collections::HashMap;
use crate::game_tree::GameTree;
use crate::game::Game;
use crate::utils::Utils;
use crate::tree_walker::TreeWalker;

pub struct StatisticsVisitor<'a, G: Game + Clone> {
    stat_nodes: HashMap<InfoState, StatisticsNode>,
    tree: &'a GameTree<G>,
}

impl<'a, G: Game + Clone> StatisticsVisitor<'a, G> {
    pub fn new(tree: &'a GameTree<G>) -> Self {
        StatisticsVisitor {
            stat_nodes: HashMap::new(),
            tree
        }
    }

    pub fn build(&mut self) {
        let game = self.tree.game.clone();
        TreeWalker::walk_tree(&game, self);

        let mut br_visitor = BestReponseVisitor::new(&mut self.stat_nodes);
        TreeWalker::walk_tree(&game, &mut br_visitor);
    }

    pub fn node_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        let updates = if stat_node.updates > 0 { stat_node.updates as f64 } else { 1.0 };

        stat_node.util_sum / updates
    }

    pub fn node_action_utils(&self, info_state: &InfoState) -> Vec<f64> {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        let updates = if stat_node.updates > 0 { stat_node.updates as f64 } else { 1.0 };

        stat_node.action_util_sums.iter().map(|x| x / updates).collect()
    }

    pub fn node_br_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        let updates = if stat_node.updates > 0 { stat_node.updates as f64 } else { 1.0 };

        stat_node.br_util / updates
    }

    pub fn node_exploitability(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        let br_util = stat_node.br_util;
        let util = stat_node.util_sum;
        let updates = if stat_node.updates > 0 { stat_node.updates as f64 } else { 1.0 };

        ((br_util - util) / updates) / util.abs() * 100.0
    }
}

impl<'a, G: Game + Clone> Visitor for StatisticsVisitor<'a, G> {
    fn visit_root_node(&mut self, info_state: &InfoState, util: f64) {
        let stat_node = self.stat_nodes.entry(info_state.clone()).or_insert(
            StatisticsNode::new(0));

        stat_node.reach_prob_sum = 1.0;
        stat_node.util_sum += util;
        stat_node.updates += 1;
    }


    fn visit_action_node(&mut self, node: &Node) {
        let stat_node = self.stat_nodes
            .entry(node.info_state()).or_insert(
                StatisticsNode::new(node.actions.len()));

        update_node(stat_node, node);
    }

    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        self.tree.average_strategy(&node.info_state())
    }
}

fn update_node(stat_node: &mut StatisticsNode, node: &Node) {
    let reach_prob = node.player_reach_prob() * node.opponent_reach_prob();
    stat_node.reach_prob_sum += reach_prob;
    stat_node.util_sum += node.util * reach_prob;

    for i in 0..stat_node.action_util_sums.len() {
        stat_node.action_util_sums[i] += node.action_utils[i] * reach_prob;
    }
}

struct BestReponseVisitor<'a> {
    stat_nodes: &'a mut HashMap<InfoState, StatisticsNode>,
}

impl<'a> BestReponseVisitor<'a> {
    pub fn new(stats: &'a mut HashMap<InfoState, StatisticsNode>) -> Self {
        BestReponseVisitor {
            stat_nodes: stats
        }
    }
}

impl<'a> Visitor for BestReponseVisitor<'a> {
    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        let action_utils = self.stat_nodes.get(&node.info_state()).unwrap().action_util_sums.clone();
        let i = Utils::arg_max(&action_utils);
        let mut action_probs = vec![0.0; node.actions.len()];
        action_probs[i] = 1.0;

        action_probs
    }

    fn visit_action_node(&mut self, node: &Node) {
        let stat_node = self.stat_nodes.get_mut(&node.info_state()).unwrap();
        let reach_prob = node.player_reach_prob() * node.opponent_reach_prob();

        stat_node.br_util += node.util * reach_prob;
        let i = Utils::arg_max(&stat_node.action_util_sums);
        stat_node.best_response = node.actions[i].clone();
    }

    fn visit_root_node(&mut self, info_state: &InfoState, util: f64) {
        let stat_node = self.stat_nodes.get_mut(&info_state).unwrap();
        stat_node.br_util += util;

        println!("BR Util: {:.4}", util);
    }
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
        assert!(statistics_visitor.node_br_util(&info_state) < 1e-6);
    }

    #[test]
    fn test_exploitable_root() {
        let mut tree: GameTree<Kuhn> = IdealKuhnBuilderVisitor::new().tree;
        let cards2 = InfoState::new(Player::OOP, HoleCards::new_with_rank(2), History::new());
        *tree.strategy_sum.get_mut(&cards2).unwrap() = vec![0.0, 1.0];
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new_empty();
        assert_ne!(statistics_visitor.node_util(&info_state), -1.0/18.0);
        assert_eq!(statistics_visitor.node_br_util(&info_state), -1.0/18.0);
        assert!(statistics_visitor.node_exploitability(&info_state) > 0.0);
    }

    #[test]
    fn test_first_child_util_sum() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        TreeWalker::walk_tree(&Kuhn::new(), &mut statistics_visitor);

        let mut ev = 0.0;
        let mut info_state = InfoState::new(Player::OOP, HoleCards::new_with_rank(1), History::new());

        ev += statistics_visitor.node_util(&info_state.clone());
        info_state.hole_cards = HoleCards::new_with_rank(2);
        ev += statistics_visitor.node_util(&info_state.clone());
        info_state.hole_cards = HoleCards::new_with_rank(3);
        ev += statistics_visitor.node_util(&info_state.clone());

        // Divide by 6 because there are 6 possible hole card combinations
        ev /= 6.0;

        assert!((ev - (-1.0/18.0)).abs() < 1e-6);
    }

    #[test]
    fn test_oop_1_x_b() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        TreeWalker::walk_tree(&Kuhn::new(), &mut statistics_visitor);

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_rank(1), History::new_from_vec(
            vec![HistoryNode::Action(Action::Check), HistoryNode::Action(Action::Bet(Bet::P(50)))]));

        assert!((statistics_visitor.node_util(&info_state) - (-2.0/3.0)).abs() < 1e-6);
    }
}