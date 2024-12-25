use crate::action::Action;
use crate::street::Street;
use crate::history_node::HistoryNode;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct History {
    history: Vec<HistoryNode>,
    street: Street,
}

impl History {
    pub fn new() -> Self {
        History {
            history: Vec::new(),
            street: Street::Preflop,
        }
    }

    pub fn new_from_vec(history: Vec<HistoryNode>) -> Self {
        History {
            street: History::find_street(&history),
            history,
        }
    }

    pub fn push_action(&mut self, action: Action) {
        self.history.push(HistoryNode::Action(action));
    }

    pub fn push_street(&mut self, street: Street) {
        self.history.push(HistoryNode::Street(street.clone()));
        self.street = street;
    }

    pub fn last(&self) -> Option<&HistoryNode> {
        self.history.last()
    }

    pub fn to_string(&self) -> String {
        self.history.iter().map(|a| a.to_string()).collect()
    }
    
    pub fn to_vec(&self) -> Vec<HistoryNode> {
        self.history.clone()
    }

    pub fn is_terminal(&self) -> bool {
        if self.history.len() < 2 {
            return false;
        }

        let last = self.history.last().unwrap().get_action();
        let second_last = self.history.get(self.history.len() - 2).unwrap().get_action();

        if last == Action::Fold || last == Action::Call {
            return true;
        } else if last == Action::Check && second_last == Action::Check {
            return true;
        } else {
            return false;
        }
    }

    pub fn street(&self) -> &Street {
        &self.street
    }

    fn find_street(history: &Vec<HistoryNode>) -> Street {
        for node in history.iter().rev() {
            if node.is_street() {
                return node.get_street();
            }
        }
        Street::Preflop
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
    use crate::board::Board;
    use crate::bet::Bet;
    

    #[test]
    fn test_new() {
        let history = History::new();
        assert_eq!(history.history, Vec::new());
        assert_eq!(history.street, Street::Preflop);
    }

    #[test]
    fn test_new_from_vec() {
        let history = History::new_from_vec(vec![HistoryNode::Action(Action::Check)]);
        assert_eq!(history.history, vec![HistoryNode::Action(Action::Check)]);
        assert_eq!(history.street, Street::Preflop);
    }

    #[test]
    fn test_is_empty_terminal() {
        let history = History::new();
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_single_action_terminal() {
        let mut history = History::new();
        history.push_action(Action::Check);
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_xx_terminal() {
        let mut history = History::new();
        history.push_action(Action::Check);
        history.push_action(Action::Check);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_fold_terminal() {
        let mut history = History::new();
        history.push_action(Action::Fold);
        history.push_action(Action::Fold);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_call_terminal() {
        let mut history = History::new();
        history.push_action(Action::Fold);
        history.push_action(Action::Call);
        assert_eq!(history.is_terminal(), true);
    }

    #[test]
    fn test_is_bet_terminal() {
        let mut history = History::new();
        history.push_action(Action::Fold);
        history.push_action(Action::Bet(Bet::P(50)));
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_check_terminal() {
        let mut history = History::new();
        history.push_action(Action::Fold);
        history.push_action(Action::Check);
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_is_raise_terminal() {
        let mut history = History::new();
        history.push_action(Action::Fold);
        history.push_action(Action::Raise(Bet::P(50)));
        assert_eq!(history.is_terminal(), false);
    }

    #[test]
    fn test_new_preflop_from_vec() {
        let history = History::new_from_vec(vec![HistoryNode::Action(Action::Check)]);
        assert_eq!(history.history, vec![HistoryNode::Action(Action::Check)]);
        assert_eq!(history.street, Street::Preflop);
    }

    #[test]
    fn test_new_flop_from_vec() {
        let history = History::new_from_vec(vec![
            HistoryNode::Action(Action::Check),
            HistoryNode::Street(Street::Flop(Board::new()))]
        );
        assert_eq!(history.street.is_flop(), true);
    }

    #[test]
    fn test_new_turn_from_vec() {
        let history = History::new_from_vec(vec![
            HistoryNode::Action(Action::Check),
            HistoryNode::Street(Street::Flop(Board::new())),
            HistoryNode::Street(Street::Turn(Board::new()))]
        );
        assert_eq!(history.street.is_turn(), true);
    }

    #[test]
    fn test_new_river_from_vec() {
        let history = History::new_from_vec(vec![
            HistoryNode::Action(Action::Check),
            HistoryNode::Street(Street::Flop(Board::new())),
            HistoryNode::Street(Street::Turn(Board::new())),
            HistoryNode::Street(Street::River(Board::new()))]
        );
        assert_eq!(history.street.is_river(), true);
    }
}