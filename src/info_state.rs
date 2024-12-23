use crate::action::Action;
use crate::history::History;
use crate::hole_cards::HoleCards;
use crate::player::Player;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoState {
    player: Player,
    hole_cards: HoleCards,
    history: History,
}

impl InfoState {
    pub fn new(hole_cards: HoleCards) -> Self {
        InfoState {
            player: Player::OOP,
            hole_cards,
            history: History::new(),
        }
    }

    pub fn history(&self) -> History {
        self.history.clone()
    }
    
    pub fn last(&self) -> Option<&Action> {
        self.history.last()
    }

    pub fn player(&self) -> Player {
        self.player
    }
    
    pub fn next_info_state(&self, action: Action, hole_cards: HoleCards) -> InfoState {
        let mut next_state = self.clone();
        next_state.history.push(action);
        next_state.player = self.player.opponent();
        next_state.hole_cards = hole_cards;
        next_state
    }
}

impl fmt::Display for InfoState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}{:}{:}", self.player, self.hole_cards, self.history)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let info_state = InfoState::new(HoleCards::new_with_rank(1));
        assert_eq!(info_state.player, Player::OOP);
        assert_eq!(info_state.hole_cards, HoleCards::new_with_rank(1));
        assert_eq!(info_state.history, History::new());
    }

    #[test]
    fn test_next_info_state() {
        let info_state = InfoState::new(HoleCards::new_with_rank(2));
        let next_card = HoleCards::new_with_rank(2);
        let next_info_state = info_state.next_info_state(Action::Check, next_card.clone());

        assert_eq!(next_info_state.player, Player::IP);
        assert_eq!(next_info_state.hole_cards, next_card);
        assert_eq!(next_info_state.history.last(), Some(&Action::Check));
    }
}