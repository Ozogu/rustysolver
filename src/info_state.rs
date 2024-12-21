use crate::action::Action;
use crate::history::History;

#[derive(Debug, Clone)]
pub struct InfoState {
    cards: Vec<usize>,
    history: History,
}

impl InfoState {
    pub fn new(cards: Vec<usize>, history: History) -> Self {
        InfoState {
            cards,
            history,
        }
    }

    pub fn new_empty() -> Self {
        InfoState {
            cards: Vec::new(),
            history: History::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.history.push(action);
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
}