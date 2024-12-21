use rand::seq::SliceRandom;
use rand::rngs::StdRng;
use crate::action::Action;
use crate::info_state::InfoState;
use crate::node::Node;

#[derive(Clone, Debug)]
pub struct Kuhn {
}

impl Kuhn {
    pub fn new() -> Self {
        Kuhn {}
    }

    pub fn deck(&self) -> Vec<usize> {
        vec![0, 1, 2]
    }

    pub fn shuffled_cards(&self, rng: &mut StdRng) -> Vec<usize> {
        let mut cards = self.deck();
        cards.shuffle(rng);
        cards
    }

    pub fn get_legal_actions(&self, info_state: &InfoState) -> Vec<Action> {
        // At root there will be no history
        let last = info_state.last().unwrap_or(&Action::Check);
        if last == &Action::Check {
            vec![Action::Check, Action::Bet(50)]
        } else { // last == Bet
            vec![Action::Call, Action::Fold]
        }
    }

    pub fn get_payoff(&self, node: &Node) -> f64 {
        let info_state = &node.info_state;
        let actions: Vec<Action> = info_state.history().to_vec();
        if actions == vec![Action::Check, Action::Check] {
            if node.player_cards() > node.opponent_cards() {
                1.0
            } else {
                -1.0
            }
        } else if actions == vec![Action::Bet(50), Action::Fold] || actions == vec![Action::Check, Action::Bet(50), Action::Fold] {
            1.0
        } else if actions == vec![Action::Bet(50), Action::Call] || actions == vec![Action::Check, Action::Bet(50), Action::Call] {
            if node.player_cards() > node.opponent_cards() {
                2.0
            } else {
                -2.0
            }
        } else {
            panic!("Invalid game state: history {:?}, player cards {:?}, opponent cards {:?}",
                actions, node.player_cards(), node.opponent_cards());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kuhn_deck() {
        let kuhn = Kuhn::new();
        assert_eq!(kuhn.deck(), vec![0, 1, 2]);
    }

    #[test]
    fn test_legal_actions_at_root() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(0);
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_check() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(0).next_info_state(Action::Check, 0);
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_bet() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(0).next_info_state(Action::Bet(50), 0);
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Call, Action::Fold]);
    }

    #[test]
    fn test_payoff_after_check_check() {
        
    }
}