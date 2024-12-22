use rand::rngs::StdRng;
use crate::action::Action;
use crate::hole_cards::HoleCards;
use crate::info_state::InfoState;
use crate::node::Node;
use crate::deck::Deck;
use crate::card::Card;
use crate::suit::Suit;

#[derive(Clone, Debug)]
pub struct Kuhn {}

impl Kuhn {
    pub fn new() -> Self {
        Kuhn {}
    }

    pub fn deck(&self) -> Deck {
        Deck::new_from_cards(vec![
            Card::new(0, Suit::Diamonds),
            Card::new(1, Suit::Diamonds),
            Card::new(2, Suit::Diamonds),
        ])
    } 

    pub fn shuffled_cards(&self, rng: &mut StdRng) -> Deck {
        let mut cards = self.deck();
        cards.shuffle(rng);
        cards.reverse();
        cards
    }

    pub fn deal(&self, rng: &mut StdRng) -> (HoleCards, HoleCards, Deck) {
        let mut cards = self.shuffled_cards(rng);
        let card1 = cards.draw().unwrap();
        let card2 = cards.draw().unwrap();

        let ip_cards = HoleCards::new_with_rank(card1.rank);
        let oop_cards = HoleCards::new_with_rank(card2.rank);
    
        (ip_cards, oop_cards, cards)
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
    fn test_legal_actions_at_root() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(HoleCards::new_with_rank(0));
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_check() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(HoleCards::new_with_rank(0)).next_info_state(Action::Check, HoleCards::new_with_rank(0));
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_bet() {
        let kuhn = Kuhn::new();
        let info_state = InfoState::new(HoleCards::new_with_rank(0)).next_info_state(Action::Bet(50), HoleCards::new_with_rank(0));
        let actions = kuhn.get_legal_actions(&info_state);
        assert_eq!(actions, vec![Action::Call, Action::Fold]);
    }
}