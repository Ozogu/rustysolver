use crate::player::Player;
use crate::hole_cards::HoleCards;
use crate::history::History;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct InfoState {
    pub player: Player,
    pub hole_cards: HoleCards,
    pub history: History,
}

impl InfoState {
    pub fn new(player: Player, hole_cards: HoleCards, history: History) -> Self {
        InfoState {
            player,
            hole_cards,
            history,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:}{:}{:}", self.player, self.hole_cards, self.history)
    }
}

impl fmt::Display for InfoState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let player = Player::IP;
        let hole_cards = HoleCards::new_with_rank(1);
        let history = History::new();
        let info_state = InfoState::new(player, hole_cards, history);

        assert_eq!(info_state.player, Player::IP);
        assert_eq!(info_state.hole_cards, HoleCards::new_with_rank(1));
        assert_eq!(info_state.history, History::new());
    }
}