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

    pub fn reach_prob(&self) -> f64 {
        self.reach_prob[&self.player()]
    }

    pub fn next_node(&self, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.info_state = next_node.info_state.next_info_state(action);
        next_node.reach_prob.insert(self.player(), self.reach_prob() * action_prob);
        next_node
    }
}