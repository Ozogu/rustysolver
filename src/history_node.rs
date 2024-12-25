use crate::action::Action;
use crate::street::Street;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum HistoryNode {
    Action(Action),
    Street(Street),
}

impl HistoryNode {
    pub fn is_street(&self) -> bool {
        match self {
            HistoryNode::Action(_) => false,
            HistoryNode::Street(_) => true,
        }
    }

    pub fn is_action(&self) -> bool {
        match self {
            HistoryNode::Action(_) => true,
            HistoryNode::Street(_) => false,
        }
    }

    pub fn get_street(&self) -> Street {
        match self {
            HistoryNode::Action(_) => Street::None,
            HistoryNode::Street(street) => street.clone(),
        }
    }

    pub fn get_action(&self) -> Action {
        match self {
            HistoryNode::Action(action) => action.clone(),
            HistoryNode::Street(_) => Action::None,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            HistoryNode::Action(action) => action.to_string(),
            HistoryNode::Street(street) => street.to_string(),
        }
    }
}