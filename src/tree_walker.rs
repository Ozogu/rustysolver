use crate::game::Game;
use crate::info_state::InfoState;
use crate::node::Node;
use crate::player::Player;
use crate::visitor::Visitor;
use rand::rngs::StdRng;
use rand::SeedableRng;

pub struct TreeWalker;

impl TreeWalker {
    pub fn walk_tree<G: Game, V: Visitor>(game: &G, visitor: &mut V) -> f64 {
        let mut ev = 0.0;
        let mut deal_count = 0;
        let rng = &mut StdRng::seed_from_u64(0);
        game.generate_deals().iter().for_each(|deal| {
            let node = Node::new(game, deal.clone());
            let node_ev = Self::iterate_tree(game, node, rng, &WalkMethod::Full, visitor);
            ev += node_ev;
            deal_count += 1;

            visitor.visit_root_node(&InfoState::new_empty(), node_ev);
        });

        return ev / deal_count as f64;
    }

    pub fn monte_carlo_iterate<G: Game, V: Visitor>(game: &G, rng: &mut StdRng, visitor: &mut V) -> f64 {
        let deal = game.deal(rng);
        let node = Node::new(game, deal);
        let node_ev = Self::iterate_tree(game, node, rng, &WalkMethod::MonteCarlo, visitor);
        visitor.visit_root_node(&InfoState::new_empty(), node_ev);

        return node_ev;
    }

    fn iterate_tree<G: Game, V: Visitor>(game: &G, mut node: Node, rng: &mut StdRng, method: &WalkMethod, visitor: &mut V) -> f64 {
        if node.is_terminal(game) {
            visitor.visit_terminal_node(&node);

            return Self::payoff(game, &node);
        } else if node.is_street_completing_action() {
            visitor.visit_street_completing_node(&node);

            // When OOP is the one completing the street,
            // node util from next action is positive.
            let sign = if node.player == Player::IP { 1.0 } else { -1.0 };
            match method {
                WalkMethod::MonteCarlo => {
                    let card = node.deck.draw().unwrap();
                    let next_street = node.history.street().next_street(card.clone());
                    let next_node = node.next_street_node(game, next_street);

                    return sign * Self::iterate_tree(game, next_node, rng, method, visitor);
                }
                WalkMethod::Full => {
                    for card in node.deck.iter() {
                        let next_street = node.history.street().next_street(card.clone());
                        let next_node = node.next_street_node(game, next_street);
                        node.util += sign * Self::iterate_tree(game, next_node, rng, method, visitor);
                    }

                    return node.util / node.deck.len() as f64;
                }
            }
        } else {
            node.action_probs = visitor.get_action_probs(&node);

            // TODO: implement walk methods
            for i in 0..node.actions.len() {
                let next_node = node.next_action_node(
                    game,
                    node.actions[i].clone(),
                    node.action_probs[i]);

                node.action_utils[i] = -Self::iterate_tree(game, next_node, rng, method, visitor);
                node.util += node.action_probs[i] * node.action_utils[i];
            }

            visitor.visit_action_node(&node);

            return node.util;
        }
    }

    fn payoff<G: Game>(game: &G, node: &Node) -> f64 {
        let won = game.player_wins(&node);
        let win_amount = node.pot.payoff(node.player, won);

        return win_amount;
    }
}

enum WalkMethod {
    MonteCarlo,
    Full,
}