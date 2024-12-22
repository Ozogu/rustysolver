use std::fmt;
use crate::action::Action;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ActionList{
    action_list: Vec<Action>
}

impl ActionList {
    pub fn new() -> Self {
        ActionList {
            action_list: Vec::new(),
        }
    }

    pub fn push(&mut self, action: Action) {
        self.action_list.push(action);
    }

    pub fn last(&self) -> Option<&Action> {
        self.action_list.last()
    }

    pub fn len(&self) -> usize {
        self.action_list.len()
    }

    pub fn get(&self, index: usize) -> Option<&Action> {
        self.action_list.get(index)
    }

    pub fn iter(&self) -> std::slice::Iter<Action> {
        self.action_list.iter()
    }

    pub fn to_vec(&self) -> Vec<Action> {
        self.action_list.clone()
    }
}

impl fmt::Display for ActionList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:}", self.action_list.iter().map(|a| a.to_string()).collect::<String>())
    }
}