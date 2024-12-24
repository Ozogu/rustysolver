#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Fold,
    Check,
    Call,
    Bet(u32),
    Raise(u32),
    None,
}

impl Action {
    pub fn to_string(&self) -> String {
        match self {
            Action::Fold => "f".to_string(),
            Action::Call => "c".to_string(),
            Action::Bet(amount) => format!("b{}", amount),
            Action::Raise(amount) => format!("r{}", amount),
            Action::Check => "x".to_string(),
            Action::None => "-".to_string(),
        }
    }
}