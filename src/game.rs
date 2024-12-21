pub trait Game {
    fn is_terminal(&self) -> bool;
    fn get_payoff(&self, player: usize) -> f64;
    // fn next_state(&self) -> Self;
    // fn get_node_actions(&self) -> Vec<String>;
}
