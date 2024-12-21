#[derive(Debug, Clone)]
pub struct InfoState {
    info_state: Vec<String>,
}

impl InfoState {
    pub fn new() -> Self {
        InfoState {
            info_state: Vec::new(),
        }
    }

    pub fn push(&mut self, action: String) {
        self.info_state.push(action);
    }

    pub fn to_string(&self) -> String {
        self.info_state.join("")
    }

    pub fn clone(&self) -> Self {
        InfoState {
            info_state: self.info_state.clone(),
        }
    }

    pub fn into_vec(self) -> Vec<String> {
        self.info_state
    }
}