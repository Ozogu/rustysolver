use std::collections::HashMap;
use crate::info_state::InfoState;
use crate::game::Game;
use crate::node::Node;
use crate::visitor::Visitor;
use crate::build_visitor::BuilderVisitor;
use crate::player::Player;
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

    fn iterate_tree<V: Visitor>(&self, mut node: Node, rng: &mut StdRng, method: &WalkMethod, visitor: &mut V) -> f64 {
        if node.is_terminal(&self.game) {
            visitor.visit_terminal_node(&node);

            return self.payoff(&node);
        } else if node.is_street_completing_action() {
            visitor.visit_street_completing_node(&node);

            // When OOP is the one completing the street,
            // node util from next action is positive.
            let sign = if node.player == Player::IP { 1.0 } else { -1.0 };
            match method {
                WalkMethod::MonteCarlo => {
                    let card = node.deck.draw().unwrap();
                    let next_street = node.history.street().next_street(card.clone());
                    let next_node = node.next_street_node(&self.game, next_street);

                    return sign * self.iterate_tree(next_node, rng, method, visitor);
                }
                WalkMethod::Full => {
                    for card in node.deck.iter() {
                        let next_street = node.history.street().next_street(card.clone());
                        let next_node = node.next_street_node(&self.game, next_street);
                        node.util += sign * self.iterate_tree(next_node, rng, method, visitor);
                    }

                    return node.util / node.deck.len() as f64;
                }
            }
        } else {
            visitor.visit_action_node(&node);
            let action_probs = visitor.get_action_probs(&node);

            // TODO: implement walk methods
            for i in 0..node.actions.len() {
                let next_node = node.next_action_node(
                    &self.game,
                    node.actions[i].clone(),
                    action_probs[i]);

                node.action_utils[i] = -self.iterate_tree(next_node, rng, method, visitor);
                node.util += action_probs[i] * node.action_utils[i];
            }

            return node.util;
        }
    }

    fn payoff(&self, node: &Node) -> f64 {
        let won = self.game.player_wins(&node);
        let win_amount = node.pot.payoff(node.player, won);

        return win_amount;
    }
}

enum WalkMethod {
    MonteCarlo,
    Full,
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