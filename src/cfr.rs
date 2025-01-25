use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::game::Game;
use crate::node::Node;
use crate::player::Player;
use crate::statistics::Statistics;
use crate::history::History;
use crate::cfr_visitor::CfrVisitor;
use crate::game_tree::GameTree;
use crate::tree_walker::TreeWalker;
use crate::tree_print_visitor::TreePrintVisitor;

pub struct CFR<G: Game + Clone> {
    game: G,
    rng: StdRng,
    tree: GameTree<G>,
}

impl<G: Game + Clone> CFR<G> {
    pub fn new(game: G) -> Self {
        let mut tree = GameTree::new(game.clone());
        tree.build();

        CFR {
            game,
            rng: StdRng::seed_from_u64(0),
            tree,
        }
    }

    pub fn train_for_iters(&mut self, iterations: usize) -> f64 {
        let mut ev = 0.0;
        let mut visitor = CfrVisitor::new(&mut self.tree.regrets, &mut self.tree.strategy_sum);
        for _ in 0..iterations {
            ev += TreeWalker::monte_carlo_iterate(&self.game, &mut self.rng, &mut visitor);
        }

        return ev / iterations as f64;
    }

    pub fn train_to_exploitability(&mut self, threshold: f64) -> f64 {
        let mut ev = 0.0;
        let mut exploitability = f64::INFINITY;
        let iteration_size = 200;
        let mut total_iterations = 0;

        while exploitability > threshold {
            ev = self.train_for_iters(iteration_size);
            let statistics = self.build_statistics();
            exploitability = statistics.node_exploitability(&History::new());
            total_iterations += iteration_size;
            println!("Total iterations: {}, exploitability: {:.2?} %", total_iterations, exploitability);
        }

        return ev;
    }

    pub fn print_strategy(&mut self) {
        let mut visitor = TreePrintVisitor::new(&self.tree);
        TreeWalker::walk_tree(&self.game, &mut visitor);

        visitor.print();
    }

    pub fn build_statistics(&self) -> Statistics {
        let mut statistics = Statistics::new();

        for deal in self.game.generate_deals() {
            let node = Node::new(&self.game, deal);
            self.iterate_statistics(node, &mut statistics);
        }

        statistics.finalize(&self.game);

        statistics
    }

    fn iterate_statistics(&self, node: Node, statistics: &mut Statistics) -> f64 {
        if node.is_terminal(&self.game) {
            let payoff = self.payoff(&node);
            statistics.update_node(&node, payoff, node.zero_utils());

            return payoff;
        } else if node.is_street_completing_action() {
            let mut node_util = 0.0;
            let sign = if node.player == Player::IP { 1.0 } else { -1.0 };

            for card in node.deck.iter() {
                let next_street = node.history.street().next_street(card.clone());
                let next_node = node.next_street_node(&self.game, next_street);

                node_util += sign * self.iterate_statistics(next_node, statistics);
            }

            return node_util / node.deck.len() as f64;
        } else {
            let strategy = self.tree.average_strategy(&node.info_state());
            let mut action_utils = node.zero_utils();
            let mut node_util = 0.0;

            for i in 0..node.actions.len() {
                let next_node = node.next_action_node(&self.game, node.actions[i].clone(), strategy[i]);
                action_utils[i] = -self.iterate_statistics(next_node, statistics);
                node_util += strategy[i] * action_utils[i];
            }

            statistics.update_node(&node, node_util, action_utils);

            node_util
        }
    }

    fn payoff(&self, node: &Node) -> f64 {
        let won = self.game.player_wins(&node);
        let win_amount = node.pot.payoff(node.player, won);

        return win_amount;
    }
}
