use std::usize;

use crate::action::Action;
use crate::history::History;
use crate::player::Player;

#[derive(Debug, Clone, PartialEq)]
pub struct InfoState {
    player: Player,
    card: usize,
    history: History,
}

impl InfoState {
    pub fn new(card: usize) -> Self {
        InfoState {
            player: Player::IP,
            card,
            history: History::new(),
        }
    }

    pub fn history(&self) -> History {
        self.history.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}{:?}", self.card, self.history)
    }

    pub fn last(&self) -> Option<&Action> {
        self.history.last()
    }

    pub fn player(&self) -> Player {
        self.player
    }
    
    pub fn next_info_state(&self, action: Action, card: usize) -> InfoState {
        let mut next_state = self.clone();
        next_state.history.push(action);
        next_state.player = self.player.opponent();
        next_state.card = card;
        next_state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info_state = InfoState::new(0);
        assert_eq!(info_state.player, Player::IP);
        assert_eq!(info_state.card, 0);
        assert_eq!(info_state.history, History::new());
    }

    #[test]
    fn test_next_info_state() {
        let info_state = InfoState::new(0);
        let next_info_state = info_state.next_info_state(Action::Check, 1);

        assert_eq!(next_info_state.player, Player::OOP);
        assert_eq!(next_info_state.card, 1);
        assert_eq!(next_info_state.history.last(), Some(&Action::Check));
    }
}