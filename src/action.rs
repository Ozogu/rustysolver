pub enum Action {
    Fold,
    Call,
    Bet(f64),
    Raise(f64),   
}