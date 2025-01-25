use std::collections::HashMap;
use crate::info_state::InfoState;
use crate::game::Game;
use crate::build_visitor::BuilderVisitor;
use crate::tree_walker::TreeWalker;

pub struct GameTree<G: Game + Clone> {
    pub regrets: HashMap<InfoState, Vec<f64>>,
    pub strategy_sum: HashMap<InfoState, Vec<f64>>,
    game: G,
}

impl<G: Game + Clone> GameTree<G> {
    pub fn new(game: G) -> Self {
        GameTree {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            game,
        }
    }

    pub fn build(&mut self) {
        let mut builder_visitor = BuilderVisitor::new();
        TreeWalker::walk_tree(&self.game, &mut builder_visitor);

        self.regrets = builder_visitor.regrets;
        self.strategy_sum = builder_visitor.strategy_sum;
    }

    pub fn print_tree(&self) {
        for (info_state, _) in self.regrets.iter() {
            println!("{}", info_state);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kuhn::Kuhn;

    #[test]
    fn test_kuhn_build() {
        let kuhn = Kuhn::new();
        let mut kuhn_tree = GameTree::new(kuhn);
        kuhn_tree.build();
        kuhn_tree.print_tree();
        assert_eq!(kuhn_tree.regrets.len(), 12);
    }
}