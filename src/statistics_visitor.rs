use crate::info_state::InfoState;
use crate::visitor::Visitor;
use crate::node::Node;
use crate::action::Action;
use std::collections::HashMap;
use crate::game_tree::GameTree;
use crate::game::Game;
use crate::utils::Utils;
use crate::tree_walker::TreeWalker;
use crate::player::Player;

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

        let mut br_visitor = BestReponseVisitor::new(&mut self.stat_nodes, Player::OOP, self.tree);
        TreeWalker::walk_tree(&game, &mut br_visitor);

        // let mut br_visitor = BestReponseVisitor::new(&mut self.stat_nodes, Player::IP, self.tree);
        // TreeWalker::walk_tree(&game, &mut br_visitor);
    }

    pub fn node_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        if stat_node.visits == 0 {
            return 0.0;
        }

        let ev = stat_node.rb_weighted_util_sum / stat_node.visits as f64;
        debug_assert!(ev.is_finite(), "Expected finite value for info state {:}, got: {:.2}", info_state, ev);

        ev
    }

    pub fn node_action_utils(&self, info_state: &InfoState) -> Vec<f64> {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        if stat_node.visits == 0 {
            return vec![0.0; stat_node.action_util_sums.len()];
        }

        stat_node.action_util_sums.iter().map(|x| x * stat_node.reach_prob / stat_node.visits as f64).collect()
    }

    pub fn node_br_util(&self, info_state: &InfoState) -> f64 {
        let stat_node = self.stat_nodes.get(info_state).unwrap();
        if stat_node.visits == 0 {
            return 0.0;
        }

        let ev = stat_node.br_util / stat_node.visits as f64;
        debug_assert!(ev.is_finite(), "Expected finite value for info state {:}, got: {:.2}", info_state, ev);

        ev
    }

    pub fn node_exploitability(&self, info_state: &InfoState) -> f64 {
        let br_util = self.node_br_util(info_state);
        let util = self.node_util(info_state);

        // BR util may be smaller than util if the node util
        // because node util is weighted by reach probability.
        // if info_state == &InfoState::new_empty() {
        //     debug_assert!(br_util >= util,
        //         "BR util should be greater than or equal to util: BR: {:.2}, Util: {:.2}", br_util, util);
        // }

        (br_util - util) / util * 100.0
    }
}

impl<'a, G: Game + Clone> Visitor for StatisticsVisitor<'a, G> {
    fn visit_root_node(&mut self, info_state: &InfoState, util: f64) {
        debug_assert!(util.is_finite(), "Expected finite value for info state {:}, got: {:.2}", info_state, util);

        let stat_node = self.stat_nodes.entry(info_state.clone()).or_insert(
            StatisticsNode::new(0));

        stat_node.rb_weighted_util_sum += util;
        stat_node.visits += 1;
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
    debug_assert!(node.util.is_finite(), "Expected finite value for node {:}, got: {:.2}", node.info_state(), node.util);
    let reach_prob = node.player_reach_prob() * node.opponent_reach_prob();

    // Only update visits if the node is reached.
    if reach_prob > 0.0 { stat_node.visits += 1; }
    stat_node.rb_weighted_util_sum += node.util * reach_prob;


    for i in 0..stat_node.action_util_sums.len() {
        stat_node.action_util_sums[i] += node.action_utils[i];
    }
}

struct BestReponseVisitor<'a, G: Game + Clone> {
    stat_nodes: &'a mut HashMap<InfoState, StatisticsNode>,
    player: Player,
    tree: &'a GameTree<G>,
}

impl<'a, G: Game + Clone> BestReponseVisitor<'a, G> {
    pub fn new(stats: &'a mut HashMap<InfoState, StatisticsNode>, player: Player, tree: &'a GameTree<G>) -> Self {
        BestReponseVisitor {
            stat_nodes: stats,
            player,
            tree,
        }
    }
}

impl<'a, G: Game + Clone> Visitor for BestReponseVisitor<'a, G> {
    fn get_action_probs(&self, node: &Node) -> Vec<f64> {
        if node.player == self.player {
            let stat_node = self.stat_nodes.get(&node.info_state()).unwrap();
            let action_utils = stat_node.action_util_sums.clone();
            let i = Utils::arg_max(&action_utils);
            let mut action_probs = vec![0.0; node.actions.len()];
            action_probs[i] = 1.0;

            action_probs
        } else {
            self.tree.average_strategy(&node.info_state())
        }
    }

    fn visit_action_node(&mut self, node: &Node) {
        let stat_node = self.stat_nodes.get_mut(&node.info_state()).unwrap();
        if node.player == self.player {
            let reach_prob = node.player_reach_prob() * node.opponent_reach_prob();

            stat_node.br_util += node.util * reach_prob;
            let i = Utils::arg_max(&stat_node.action_util_sums);
            stat_node.best_response = node.actions[i].clone();
        };
    }

    fn visit_root_node(&mut self, info_state: &InfoState, util: f64) {
        if self.player == Player::OOP {
            let stat_node = self.stat_nodes.get_mut(&info_state).unwrap();

            stat_node.br_util += util;
        }
    }
}

#[derive(Clone, Debug)]
struct StatisticsNode {
    pub rb_weighted_util_sum: f64,
    pub action_util_sums: Vec<f64>,
    pub reach_prob: f64,
    pub br_util: f64,
    pub best_response: Action,
    pub visits: u32,
}

impl StatisticsNode {
    fn new(actions: usize) -> Self {
        StatisticsNode {
            rb_weighted_util_sum: 0.0,
            action_util_sums: vec![0.0; actions],
            reach_prob: 0.0,
            br_util: 0.0,
            best_response: Action::None,
            visits: 0,
        }
    }

    fn log(&self) {
        println!("Reach prob weighter util sum: {:.2}", self.rb_weighted_util_sum);
        println!("Action util sums: {:.2?}", self.action_util_sums);
        println!("BR Util: {:.2}", self.br_util);
        println!("Best response: {:?}", self.best_response);
        println!("visits: {}", self.visits);
        println!("_________________");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::history_node::HistoryNode;
    use crate::hole_cards::HoleCards;
    use crate::info_state::InfoState;
    use crate::ideal_kuhn_builder_visitor::IdealKuhnBuilderVisitor;
    use crate::player::Player;
    use crate::bet::Bet;
    use crate::history::History;
    use crate::kuhn::Kuhn;

    fn create_exploitable_IP_2_strategy() -> GameTree<Kuhn> {
        let mut tree: GameTree<Kuhn> = IdealKuhnBuilderVisitor::new().tree;
        // Create suboptimal strategy for player 1
        let cards2 = InfoState::new(Player::IP, HoleCards::new_with_ranks(2, 2),
            History::new_from_vec(vec![HistoryNode::Action(Action::Check)]));
        *tree.strategy_sum.get_mut(&cards2).unwrap() = vec![0.0, 1.0];

        tree
    }

    #[test]
    fn test_ideal_root() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new_empty();
        let ideal_ev = -1.0/18.0;

        debug_assert!((statistics_visitor.node_util(&info_state) - ideal_ev).abs() < 1e-6,
            "Expected: -1/18, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - ideal_ev).abs() < 1e-6,
            "Expected: -1/18, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_exploitable_root() {
        let tree = create_exploitable_IP_2_strategy();
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        // EV of 1 = -1
        // EV of 2 = -1/3
        // EV of 3 = 5/3
        // Avg EV = (-1 + -1/3 + 5/3) / 3 = 1/9

        let info_state = InfoState::new_empty();
        debug_assert!((statistics_visitor.node_util(&info_state) - -1.0/18.0).abs() < 1e-6,
            "Expected: 1/18, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - 1.0/9.0).abs() < 1e-6,
            "Expected: 1/9, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) - 0.0 < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_exploitable_oop_1() {
        let tree = create_exploitable_IP_2_strategy();
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        // We cannot do anything vs. villain leak. Util should be same
        // Best response EV
        // 1 vs 2 X line util: 1 * 1 * -1 = -1
        // 1 vs 3 X line util: 1 * 1 * -1 = -1
        // Average util = (-1 + -1) / 2 = -1

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(1, 1), History::new());
        debug_assert!((statistics_visitor.node_util(&info_state) - -1.0).abs() < 1e-6,
            "Expected: -1.0, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - -1.0).abs() < 1e-6,
            "Expected: -1.0, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_exploitable_oop_2() {
        let tree = create_exploitable_IP_2_strategy();
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        // Nothing changed in this part of the tree, should be same as ideal.

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(2, 2), History::new());
        debug_assert!((statistics_visitor.node_util(&info_state) - -1.0/3.0).abs() < 1e-6,
            "Expected: -1/3, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - -1.0/3.0).abs() < 1e-6,
            "Expected: -1/3, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_exploitable_oop_3() {
        let tree = create_exploitable_IP_2_strategy();
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        // Villain bluffing onto us should give significant EV boost
        // Best response EV
        // 3 vs 1 X line util: 1 * ((2/3 * 1) + 1/3 * (1 * 2)) = 4/3
        // 3 vs 2 X line util: 1 * 1 * 1 * 2 = 2
        // Average util = (4/3 + 2) / 2 = 5/3

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(3, 3), History::new());

        debug_assert!((statistics_visitor.node_util(&info_state) - 7.0/6.0).abs() < 1e-6,
            "Expected: 7/6, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - 5.0/3.0).abs() < 1e-6,
            "Expected: 5/3, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) - 42.8571 < 1e-4,
            "Expected 42.8571 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_first_child_util_sum() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let mut ev = 0.0;
        let mut info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(1, 1), History::new());

        ev += statistics_visitor.node_util(&info_state.clone());
        info_state.hole_cards = HoleCards::new_with_ranks(2, 2);
        ev += statistics_visitor.node_util(&info_state.clone());
        info_state.hole_cards = HoleCards::new_with_ranks(3, 3);
        ev += statistics_visitor.node_util(&info_state.clone());

        // Each card will return average utility of 2 nodes.
        // Tree has 6 nodes. Dividing by 3 to get average utility.
        ev /= 3.0;

        debug_assert!((ev - (-1.0/18.0)).abs() < 1e-6,
            "Expected: -1/18, got: {:.4}", ev);
    }

    #[test]
    fn test_oop_1_x_b() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(1, 1), History::new_from_vec(
            vec![HistoryNode::Action(Action::Check), HistoryNode::Action(Action::Bet(Bet::P(50)))]));

        debug_assert!((statistics_visitor.node_util(&info_state) - (-2.0/3.0)).abs() < 1e-6,
            "Expected: -2/3, got: {:.4}", statistics_visitor.node_util(&info_state));
    }

    #[test]
    fn test_oop_1() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(1, 1), History::new());

        // a = 1/3
        // 1 vs 2 B line util: a * ((2/3 * 1) + (1/3 * -2)) = 0
        // 1 vs 2 X line util: (1-a) * (1 * -1) = -2/3
        // util sum = 0 + -2/3 = -2/3

        // 1 vs 3 B line util: a * (1 * -2) = -2/3
        // 1 vs 3 X line util: (1-a) * (1 * -1) = -2/3
        // util sum = -2/3 + -2/3 = -4/3

        // average util = (-4/3 + -2/3) / 2 = -1


        debug_assert!((statistics_visitor.node_util(&info_state) - (-1.0)).abs() < 1e-6,
            "Expected: -1.0, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - (-1.0)).abs() < 1e-6,
            "Expected: -1.0, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_oop_2() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(2, 2), History::new());

        // a = 1/3
        // 2 vs 1 B line util: 1 * (2/3 * 1) + (1/3 * ((2/3 - a) * -1 + (a + 1/3) * 2)) = 1
        // 2 vs 1 X line util: 0 * (???) = 0
        // util sum = 1 + 0 = 1

        // 2 vs 3 B line util: 1 * (1 * (((2/3 - a) * -1 + (a + 1/3) * -2))) = -5/3
        // 2 vs 3 X line util: 0 * (???) = 0
        // util sum = -2 + 0 = -2

        // average util = (1 + -5/3) / 2 = -1/3

        debug_assert!((statistics_visitor.node_util(&info_state) - (-1.0/3.0)).abs() < 1e-6,
            "Expected: -1/3, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - (-1.0/3.0)).abs() < 1e-6,
            "Expected: -1/3, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    #[test]
    fn test_oop_3() {
        let tree = IdealKuhnBuilderVisitor::new().tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(3, 3), History::new());

        // a = 1/3
        // 3 vs 1 B line util: 3*a * (1 * 1) = 1
        // 3 vs 1 X line util: 1-3a * (???) = 0
        // util sum = 1 + 0 = 1

        // 3 vs 2 B line util: 3*a * (2/3 * 1 + 1/3 * 2) = 4/3
        // 3 vs 2 X line util: 1-3a * (???) = 0
        // util sum = 4/3 + 0 = 4/3

        // average util = (1 + 4/3) / 2 = 7/6

        debug_assert!((statistics_visitor.node_util(&info_state) - (7.0/6.0)).abs() < 1e-6,
            "Expected: 7/6, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - (7.0/6.0)).abs() < 1e-6,
            "Expected: 7/6, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }

    // Total EV = (-1 + -1/3 + 7/6) / 3 = -1/18

    #[test]
    fn test_oop_3_a0() {
        let tree = IdealKuhnBuilderVisitor::new_a(0.0).tree;
        let mut statistics_visitor = StatisticsVisitor::new(&tree);
        statistics_visitor.build();

        let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(3, 3), History::new());

        // a = 0
        // 3 vs 1 B line util: 3*a * (1 * 1) = 0
        // 3 vs 1 X line util: (1-3*a) * ((2/3 * 1) + (1/3 * (1 * 2))) = 4/3
        // util sum = 0 + 4/3 = 4/3

        // 3 vs 2 B line util: 3*a * ((2/3 * 1) + 1/3 * 2) = 0
        // 3 vs 2 X line util: (1-3*a) * (1 * 1) = 1
        // util sum = 0 + 1 = 1

        // average util = (4/3 + 1) / 2 = 7/6

        debug_assert!((statistics_visitor.node_util(&info_state) - (7.0/6.0)).abs() < 1e-6,
            "Expected: 7/6, got: {:.4}", statistics_visitor.node_util(&info_state));
        debug_assert!((statistics_visitor.node_br_util(&info_state) - (7.0/6.0)).abs() < 1e-6,
            "Expected: 7/6, got: {:.4}", statistics_visitor.node_br_util(&info_state));
        debug_assert!(statistics_visitor.node_exploitability(&info_state) < 1e-6,
            "Expected 0 exploitability, got: {:.4}", statistics_visitor.node_exploitability(&info_state));
    }
}