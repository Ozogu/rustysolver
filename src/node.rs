use std::collections::HashMap;
use crate::hole_cards::HoleCards;
use crate::player::Player;
use crate::info_state::InfoState;
use crate::action::Action;

#[derive(Clone, Debug)]
pub struct Node {
    pub info_state: InfoState,
    pub reach_prob: HashMap<Player, f64>,
    pub cards: HashMap<Player, HoleCards>,
}

impl Node {
    pub fn new(ip_cards: HoleCards, oop_cards: HoleCards) -> Node {
        Node {
            info_state: InfoState::new(ip_cards.clone()),
            reach_prob: HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]),
            cards: HashMap::from([(Player::IP, ip_cards), (Player::OOP, oop_cards)]),
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

    pub fn next_node(&self, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.info_state = next_node.info_state.next_info_state(action, self.opponent_cards());
        next_node.reach_prob.insert(self.player(), self.player_reach_prob() * action_prob);
        next_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        assert_eq!(node.reach_prob[&Player::IP], 1.0);
        assert_eq!(node.reach_prob[&Player::OOP], 1.0);
    }

    #[test]
    fn test_reach_prob() {
        let node = Node::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        assert_eq!(node.player_reach_prob(), 1.0);
        assert_eq!(node.opponent_reach_prob(), 1.0);

        // in next node the previous player's (opponent) reach prob is multiplied by the action prob
        let next_node = node.next_node(Action::Check, 0.5);
        assert_eq!(next_node.player_reach_prob(), 1.0);
        assert_eq!(next_node.opponent_reach_prob(), 0.5);
        
        let next_node = next_node.next_node(Action::Check, 0.25);
        assert_eq!(next_node.opponent_reach_prob(), 0.25);
        assert_eq!(next_node.player_reach_prob(), 0.5);
    }

    #[test]
    fn test_next_node() {
        let node = Node::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let next_node = node.next_node(Action::Check, 0.5);
        assert_eq!(next_node.reach_prob[&Player::IP], 0.5);
        assert_eq!(next_node.reach_prob[&Player::OOP], 1.0);
    }

    #[test]
    fn test_player_cards() {
        let ip_cards = HoleCards::new_with_rank(1);
        let oop_cards = HoleCards::new_with_rank(2);
        let node = Node::new(ip_cards.clone(), oop_cards.clone());
        assert_eq!(node.player_cards(), ip_cards);
        assert_eq!(node.opponent_cards(), oop_cards);

        let next_node = node.next_node(Action::Check, 1.0);
        assert_eq!(next_node.player_cards(), oop_cards);
        assert_eq!(next_node.opponent_cards(), ip_cards);
    }
}
