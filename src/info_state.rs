use crate::action::Action;
use crate::history::History;
use crate::player::Player;

#[derive(Debug, Clone)]
pub struct InfoState {
    player: Player,
    cards: Vec<usize>,
    history: History,
}

impl InfoState {
    pub fn new(cards: Vec<usize>) -> Self {
        InfoState {
            player: Player::IP,
            cards,
            history: History::new(),
        }
    }

    pub fn history(&self) -> History {
        self.history.clone()
    }

    pub fn to_string(&self) -> String {
        format!("{:?}{:?}", self.cards, self.history)
    }

    pub fn last(&self) -> Option<&Action> {
        self.history.last()
    }

    pub fn player(&self) -> Player {
        self.player
    }
    
    pub fn next_info_state(&self, action: Action) -> InfoState {
        let mut next_state = self.clone();
        next_state.history.push(action);
        next_state.player = self.player.opponent();
        next_state
    }
}