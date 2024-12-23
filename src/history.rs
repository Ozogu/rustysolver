use crate::action::Action;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct History {
    history: Vec<Action>
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

impl fmt::Display for History {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.history.iter().map(|a| a.to_string()).collect::<String>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let history = History::new();
        assert_eq!(history.history, Vec::new());
    }

    #[test]
    fn test_is_empty_terminal() {
        let history = History::new();
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_single_action_terminal() {
        let mut history = History::new();
        history.push(Action::Check);
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_xx_terminal() {
        let mut history = History::new();
        history.push(Action::Check);
        history.push(Action::Check);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_fold_terminal() {
        let mut history = History::new();
        history.push(Action::Fold);
        history.push(Action::Fold);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_call_terminal() {
        let mut history = History::new();
        history.push(Action::Fold);
        history.push(Action::Call);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_bet_terminal() {
        let mut history = History::new();
        history.push(Action::Fold);
        history.push(Action::Bet(50));
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_check_terminal() {
        let mut history = History::new();
        history.push(Action::Fold);
        history.push(Action::Check);
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_raise_terminal() {
        let mut history = History::new();
        history.push(Action::Fold);
        history.push(Action::Raise(50));
        assert_eq!(history.is_terminal(), false);
    }
}