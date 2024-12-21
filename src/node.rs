use std::collections::HashMap;
use crate::player::Player;
use crate::action::Action;

struct Node {
    player: Player,
    actions: Vec<Action>,
    reach_prob: HashMap<Player, f64>,
}