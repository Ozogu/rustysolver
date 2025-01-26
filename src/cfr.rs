use rand::SeedableRng;
use rand::rngs::StdRng;
use crate::game::Game;
use crate::cfr_visitor::CfrVisitor;
use crate::game_tree::GameTree;
use crate::info_state::InfoState;
use crate::tree_walker::TreeWalker;
use crate::tree_print_visitor::TreePrintVisitor;
use crate::statistics_visitor::StatisticsVisitor;

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
            exploitability = statistics.node_exploitability(&InfoState::new_empty());
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

    pub fn build_statistics(&self) -> StatisticsVisitor<G> {
        let mut visitor = StatisticsVisitor::new(&self.tree);
        TreeWalker::walk_tree(&self.game, &mut visitor);

        visitor
    }
}
