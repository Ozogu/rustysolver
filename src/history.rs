use crate::action::Action;

#[derive(Clone, Debug)]
pub struct History {
    history: Vec<Action>,
}

impl History {
    pub fn new() -> Self {
        History {
            history: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.history.push(action);
    }

    pub fn last(&self) -> Option<&Action> {
        self.history.last()
    }

    pub fn to_string(&self) -> String {
        self.history.iter().map(|a| a.to_string()).collect()
    }
    
    pub fn to_vec(&self) -> Vec<Action> {
        self.history.clone()
    }

    pub fn is_terminal(&self) -> bool {
        if self.history.len() < 2 {
            return false;
        }

        let last = self.history.last().unwrap();
        let second_last = self.history.get(self.history.len() - 2).unwrap();

        if last == &Action::Fold || last == &Action::Call {
            return true;
        } else if last == &Action::Check && second_last == &Action::Check {
            return true;
        } else {
            return false;
        }
    }
}