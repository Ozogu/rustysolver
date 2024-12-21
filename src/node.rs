use std::collections::HashMap;
use crate::player::Player;
use crate::info_state::InfoState;
use crate::action::Action;

#[derive(Clone, Debug)]
pub struct Node {
    pub info_state: InfoState,
    pub reach_prob: HashMap<Player, f64>,
}

impl Node {
    pub fn new(cards: Vec<usize>) -> Self {
        Node {
            info_state: InfoState::new(cards),
            reach_prob: HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]),
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

    pub fn next_node(&self, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.info_state = next_node.info_state.next_info_state(action);
        next_node.reach_prob.insert(self.player(), self.player_reach_prob() * action_prob);
        next_node
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let node = Node::new(Vec::new());
        assert_eq!(node.reach_prob[&Player::IP], 1.0);
        assert_eq!(node.reach_prob[&Player::OOP], 1.0);
    }

    #[test]
    fn test_reach_prob() {
        let node = Node::new(Vec::new());
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
        let cards = vec![1, 2, 3];
        let node = Node::new(cards);
        let next_node = node.next_node(Action::Check, 0.5);
        assert_eq!(next_node.reach_prob[&Player::IP], 0.5);
        assert_eq!(next_node.reach_prob[&Player::OOP], 1.0);
    }
}
