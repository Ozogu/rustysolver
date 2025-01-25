use std::collections::HashMap;
use crate::info_state::InfoState;
use crate::game::Game;
use crate::node::Node;
use crate::visitor::Visitor;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct GameTree<G: Game> {
    regrets: HashMap<InfoState, Vec<f64>>,
    strategy_sum: HashMap<InfoState, Vec<f64>>,
    game: G,
}

impl<G: Game> GameTree<G> {
    pub fn new(game: G) -> Self {
        GameTree {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
            game,
        }
    }

    pub fn build(&mut self) {
        let mut builder_visitor = BuilderVisitor::new();
        self.walk_tree(&mut builder_visitor);

        self.regrets = builder_visitor.regrets;
        self.strategy_sum = builder_visitor.strategy_sum;
    }

    pub fn print_tree(&self) {
        for (info_state, _) in self.regrets.iter() {
            println!("{}", info_state);
        }
    }

    pub fn walk_tree<V: Visitor>(&self, visitor: &mut V) {
        let rng = &mut StdRng::seed_from_u64(0);
        self.game.generate_deals().iter().for_each(|deal| {
            let node = Node::new(&self.game, deal.clone());
            self.iterate_tree(node, rng, &WalkMethod::Full, visitor);
        });
    }

    pub fn monte_carlo_iterate<V: Visitor>(&self, rng: &mut StdRng, visitor: &mut V) {
        let deal = self.game.deal(rng);
        let node = Node::new(&self.game, deal);
        self.iterate_tree(node, rng, &WalkMethod::MonteCarlo, visitor);
    }

    fn iterate_tree<V: Visitor>(&self, node: Node, rng: &mut StdRng, method: &WalkMethod, visitor: &mut V) {
        if node.is_terminal(&self.game) {
            visitor.visit_terminal_node(&node);

            return;
        } else if node.is_street_completing_action() {
            visitor.visit_street_completing_node(&node);

            match method {
                WalkMethod::MonteCarlo => {
                    let card = node.deck.get(0).unwrap();
                    let next_street = node.history.street().next_street(card.clone());
                    let mut next_node = node.next_street_node(&self.game, next_street);
                    next_node.deck.draw();

                    self.iterate_tree(next_node, rng, method, visitor);
                }
                WalkMethod::Full => {
                    for card in node.deck.iter() {
                        let next_street = node.history.street().next_street(card.clone());
                        let next_node = node.next_street_node(&self.game, next_street);
                        self.iterate_tree(next_node, rng, method, visitor);
                    }
                }
            }

            return;
        } else {
            visitor.visit_action_node(&node);

            // TODO: implement walk methods
            for action in node.actions.iter() {
                let new_node = node.next_action_node(&self.game, action.clone(), 1.0);

                self.iterate_tree(new_node, rng, method, visitor);
            }
        }
    }
}

struct BuilderVisitor {
    pub regrets: HashMap<InfoState, Vec<f64>>,
    pub strategy_sum: HashMap<InfoState, Vec<f64>>,
}

enum WalkMethod {
    MonteCarlo,
    Full,
}

impl BuilderVisitor {
    fn new() -> Self {
        BuilderVisitor {
            regrets: HashMap::new(),
            strategy_sum: HashMap::new(),
        }
    }

    fn add_node(&mut self, node: &Node) {
        let info_state = node.info_state().clone();
        self.regrets.entry(info_state.clone()).or_insert(node.zero_utils());
        self.strategy_sum.entry(info_state).or_insert(node.zero_utils());
    }
}

impl Visitor for BuilderVisitor {
    fn visit_terminal_node(&mut self, node: &Node) {
        return;
    }

    fn visit_street_completing_node(&mut self, node: &Node) {
        self.add_node(node);
    }

    fn visit_action_node(&mut self, node: &Node) {
        self.add_node(node);
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