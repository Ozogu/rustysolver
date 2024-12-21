use std::collections::HashMap;
use crate::player::Player;
use crate::action::Action;
use crate::info_state::InfoState;

struct Node {
    player: Player,
    actions: Vec<Action>,
    reach_prob: HashMap<Player, f64>,
    info_state: InfoState,
}