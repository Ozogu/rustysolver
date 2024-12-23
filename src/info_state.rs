use crate::action::Action;
use crate::history::History;
use crate::hole_cards::HoleCards;
use crate::player::Player;
use crate::board::Board;
use crate::player_cards::PlayerCards;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InfoState {
    history: History,
    player: Player,
    cards: PlayerCards,
    board: Board,
}

impl InfoState {
    pub fn new(cards: PlayerCards) -> Self {
        InfoState {
            history: History::new(),
            player: Player::OOP,
            cards,
            board: Board::new(),
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

    pub fn player_cards(&self) -> HoleCards {
        self.cards.get(self.player)
    }

    pub fn opponent_cards(&self) -> HoleCards {
        self.cards.get(self.player.opponent())
    }
    
    pub fn next_info_state(&self, action: Action) -> InfoState {
        let mut next_state = self.clone();
        next_state.history.push(action);
        next_state.player = self.player.opponent();
        next_state
    }
}

impl fmt::Display for InfoState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}{:}{:}", self.player, self.player_cards(), self.history)
    }
}

impl Hash for InfoState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.history.hash(state);
        self.player.hash(state);
        self.player_cards().hash(state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn test_new() {
        let cards = PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let info_state = InfoState::new(cards.clone());
        assert_eq!(info_state.player, Player::OOP);
        assert_eq!(info_state.cards, cards);
        assert_eq!(info_state.history, History::new());
    }

    #[test]
    fn test_next_info_state() {
        let cards = PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let info_state = InfoState::new(cards);
        let next_info_state = info_state.next_info_state(Action::Check);

        assert_eq!(next_info_state.player, Player::IP);
        assert_eq!(next_info_state.history.last(), Some(&Action::Check));
    }

    #[test]
    fn test_player_cards() {
        let cards = PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let info_state = InfoState::new(cards);
        assert_eq!(info_state.player_cards(), HoleCards::new_with_rank(2));
        assert_eq!(info_state.opponent_cards(), HoleCards::new_with_rank(1));
    }

    #[test]
    fn test_next_node_cards() {
        let cards = PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2));
        let info_state = InfoState::new(cards);
        let next_info_state = info_state.next_info_state(Action::Check);
        assert_eq!(next_info_state.player_cards(), HoleCards::new_with_rank(1));
        assert_eq!(next_info_state.opponent_cards(), HoleCards::new_with_rank(2));
    }

    #[test]
    fn test_hashing_ignore_opponent_cards() {
        let info_state1 = InfoState::new(PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)));
        let mut hasher1 = DefaultHasher::new();
        info_state1.hash(&mut hasher1);
        let hash1 = hasher1.finish();
        
        let info_state2 = InfoState::new(PlayerCards::new(HoleCards::new_with_rank(3), HoleCards::new_with_rank(2)));
        let mut hasher2 = DefaultHasher::new();
        info_state2.hash(&mut hasher2);
        let hash2 = hasher2.finish();

        assert_eq!(hash1, hash2);
    }
}