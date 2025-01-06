use crate::bet::Bet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(Bet),
    Raise(Bet),
    None,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Fold => "F".to_string(),
            Action::Call => "C".to_string(),
            Action::Bet(bet) => format!("B{}", bet),
            Action::Raise(bet) => format!("R{}", bet),
            Action::Check => "X".to_string(),
            Action::None => "-".to_string(),
        }
    }
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_string())
    }
}