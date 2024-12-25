use std::collections::HashMap;
use crate::history::History;
use crate::node::Node;
use crate::game::Game;
use crate::action::Action;
use crate::info_state::InfoState;

#[derive(Clone, Debug)]
pub struct Statistics {
    nodes: HashMap<InfoState, StatisticsNode>,
}

impl Statistics {
    pub fn new() -> Self {
        Statistics {
            nodes: HashMap::new(),
        }
    }

    pub fn update_node(&mut self, node: &Node, util: f64, action_utils: Vec<f64>) {
        let info_state = node.info_state().clone();
        let stat_node = self.nodes.entry(info_state).or_insert(StatisticsNode {
            util: 0.0,
            action_utils: node.zero_utils(),
            br_util: 0.0,
            best_response: Action::None,
            updates: 0,
        });

        stat_node.updates += 1;
        stat_node.util += util;
    
        for i in 0..stat_node.action_utils.len() {
            stat_node.action_utils[i] += action_utils[i];
        }
    }
    
    pub fn finalize<G: Game>(&mut self, game: &G) {
        for (info_state, node) in self.nodes.iter_mut() {
            node.util /= node.updates as f64;
            for i in 0..node.action_utils.len() {
                node.action_utils[i] /= node.updates as f64;
            }

            let (br_util, best_response) =
                Self::find_best_reponse(game, info_state, node);
    
            node.br_util = br_util;
            node.best_response = best_response;
        }
    }

    pub fn info_state_util(&self, info_state: &InfoState) -> f64 {
        self.nodes.get(info_state).unwrap().util
    }

    pub fn node_util(&self, history: &History) -> f64 {
        let mut node_util = 0.0;
        let mut updates = 0;
        for (info_state, node) in self.nodes.iter() {
            if &info_state.history == history {
                node_util += node.util;
                updates += 1;
            }
        }

        node_util / updates as f64
    }

    pub fn action_utils(&self, info_state: &InfoState) -> Vec<f64> {
        self.nodes.get(info_state).unwrap().action_utils.clone()
    }

    pub fn info_state_br_util(&self, info_state: &InfoState) -> f64 {
        self.nodes.get(info_state).unwrap().br_util
    }

    pub fn node_br_util(&self, history: &History) -> f64 {
        let mut node_br_util = 0.0;
        let mut updates = 0;
        for (info_state, node) in self.nodes.iter() {
            if &info_state.history == history {
                node_br_util += node.br_util;
                updates += 1;
            }
        }

        node_br_util / updates as f64
    }

    pub fn best_response(&self, info_state: &InfoState) -> Action {
        self.nodes.get(info_state).unwrap().best_response.clone()
    }

    pub fn to_map(&self) -> HashMap<InfoState, StatisticsNode> {
        self.nodes.clone()
    }

    pub fn node_exploitability(&self, history: &History) -> f64 {
        let mut node_util = 0.0;
        let mut node_br_util = 0.0;
        let mut updates = 0;
        for (info_state, node) in self.nodes.iter() {
            if &info_state.history == history {
                node_util += node.util;
                node_br_util += node.br_util;
                updates += 1;
            }
        }

        node_util /= updates as f64;
        node_br_util /= updates as f64;

        (node_br_util - node_util) / node_util.abs() * 100.0
    }

    fn find_best_reponse<G: Game>(game: &G, info_state: &InfoState, node: &StatisticsNode) -> (f64, Action) {
        let mut br_util = f64::NEG_INFINITY;
        let mut best_reponse = Action::None;        
        let node_actions = game.legal_actions(&info_state.history);
        for (i, util) in node.action_utils.iter().enumerate() {
            if util > &br_util {
                br_util = *util;
                best_reponse = node_actions[i].clone();
            }
        }

        (br_util, best_reponse)
    }
}

#[derive(Clone, Debug)]
pub struct StatisticsNode {
    pub util: f64,
    pub action_utils: Vec<f64>,
    pub br_util: f64,
    pub best_response: Action,
    pub updates: u32,
}