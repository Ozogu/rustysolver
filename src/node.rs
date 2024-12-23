use std::collections::HashMap;
use crate::hole_cards::HoleCards;
use crate::game::Game;
use crate::player::Player;
use crate::info_state::InfoState;
use crate::action::Action;
use crate::pot::Pot;

#[derive(Clone, Debug)]
pub struct Node {
    pub info_state: InfoState,
    pub reach_prob: HashMap<Player, f64>,
    pub cards: HashMap<Player, HoleCards>,
    pub actions: Vec<Action>,
    pub pot: Pot,
}

impl Node {
    pub fn new<G: Game>(game: &G, ip_cards: HoleCards, oop_cards: HoleCards) -> Node {
        let info_state = InfoState::new(oop_cards.clone());
        Node {
            actions: game.get_legal_actions(&info_state),
            info_state,
            reach_prob: HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]),
            cards: HashMap::from([(Player::IP, ip_cards), (Player::OOP, oop_cards)]),
            pot: game.initial_pot(),
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.info_state.history().is_terminal()
    }
    
    pub fn player(&self) -> Player {
        self.info_state.player()
    }

    pub fn player_reach_prob(&self) -> f64 {
        self.reach_prob[&self.player()]
    }

    pub fn opponent_reach_prob(&self) -> f64 {
        self.reach_prob[&self.player().opponent()]
    }

    pub fn player_cards(&self) -> HoleCards {
        self.cards[&self.player()].clone()
    }

    pub fn opponent_cards(&self) -> HoleCards {
        self.cards[&self.player().opponent()].clone()
    }

    pub fn zero_utils(&self) -> Vec<f64> {
        vec![0.0; self.actions.len()]
    }

    pub fn next_node<G: Game>(&self, game: &G, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.info_state = next_node.info_state.next_info_state(action, self.opponent_cards());
        next_node.reach_prob.insert(self.player(), self.player_reach_prob() * action_prob);
        next_node.actions = game.get_legal_actions(&next_node.info_state);
        next_node.pot.update(self.player(), action);
        next_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kuhn::Kuhn;

    #[test]
    fn test_new() {
        let node = Node::new(&Kuhn::new(), HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        assert_eq!(node.reach_prob[&Player::IP], 1.0);
        assert_eq!(node.reach_prob[&Player::OOP], 1.0);
    }

    #[test]
    fn test_reach_prob() {
        let node = Node::new(&Kuhn::new(), HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        assert_eq!(node.player_reach_prob(), 1.0);
        assert_eq!(node.opponent_reach_prob(), 1.0);

        // in next node the previous player's (opponent) reach prob is multiplied by the action prob
        let next_node = node.next_node(&Kuhn::new(), Action::Check, 0.5);
        assert_eq!(next_node.player_reach_prob(), 1.0);
        assert_eq!(next_node.opponent_reach_prob(), 0.5);
        
        let next_node = next_node.next_node(&Kuhn::new(), Action::Check, 0.25);
        assert_eq!(next_node.opponent_reach_prob(), 0.25);
        assert_eq!(next_node.player_reach_prob(), 0.5);
    }

    #[test]
    fn test_next_node() {
        let node = Node::new(&Kuhn::new(), HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let next_node = node.next_node(&Kuhn::new(), Action::Bet(50), 0.5);
        assert_eq!(next_node.reach_prob[&Player::OOP], 0.5);
        assert_eq!(next_node.reach_prob[&Player::IP], 1.0);
        assert_eq!(next_node.actions, Kuhn::new().get_legal_actions(&next_node.info_state));
        assert_eq!(next_node.pot.total(), node.pot.total() + 1.0);
        assert_eq!(next_node.pot.contributions(), HashMap::from([(Player::IP, 1.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_player_cards() {
        let ip_cards = HoleCards::new_with_rank(1);
        let oop_cards = HoleCards::new_with_rank(2);
        let node = Node::new(&Kuhn::new(), ip_cards.clone(), oop_cards.clone());
        assert_eq!(node.player_cards(), oop_cards);
        assert_eq!(node.opponent_cards(), ip_cards);

        let next_node = node.next_node(&Kuhn::new(), Action::Check, 1.0);
        assert_eq!(next_node.player_cards(), ip_cards);
        assert_eq!(next_node.opponent_cards(), oop_cards);
    }
}
