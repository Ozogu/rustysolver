use rand::rngs::StdRng;
use crate::action::Action;
use crate::hole_cards::HoleCards;
use crate::info_state::InfoState;
use crate::node::Node;
use crate::deck::Deck;
use crate::card::Card;
use crate::suit::Suit;
use crate::pot::Pot;

#[derive(Clone, Debug)]
pub struct Kuhn {}

impl Kuhn {
    pub fn new() -> Self {
        Kuhn {}
    }

    pub fn initial_pot(&self) -> Pot {
        Pot::new(1.0, 1.0)
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
        match last {
            Action::Check | Action::Call => vec![Action::Check, Action::Bet(50)],
            Action::Bet(50) => vec![Action::Call, Action::Fold],
            _ => vec![],
        }
    }

    pub fn player_wins(&self, node: &Node) -> Option<bool> {
        let last = node.info_state.last().unwrap();
        match last {
            Action::Fold => Some(true),
            Action::Check | Action::Call => {
            let result = node.player_cards().partial_cmp(&node.opponent_cards());
            match result {
                Some(std::cmp::Ordering::Greater) => Some(true),
                Some(std::cmp::Ordering::Less) => Some(false),
                _ => None,
            }
            }
            _ => panic!("Invalid action: {:?}", last),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::player::Player;
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

    #[test]
    fn test_player_wins_xx() {
        let kuhn = Kuhn::new();
        let node = Node::new(&kuhn, HoleCards::new_with_rank(0), HoleCards::new_with_rank(1));
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Check, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbf() {
        let kuhn = Kuhn::new();
        let node = Node::new(&kuhn, HoleCards::new_with_rank(0), HoleCards::new_with_rank(1));
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::IP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbc() {
        let kuhn = Kuhn::new();
        let node = Node::new(&kuhn, HoleCards::new_with_rank(0), HoleCards::new_with_rank(1));
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::IP, Some(false)));
    }

    #[test]
    fn test_player_wins_bf() {
        let kuhn = Kuhn::new();
        let node = Node::new(&kuhn, HoleCards::new_with_rank(0), HoleCards::new_with_rank(1));
        let next_node = node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_bc() {
        let kuhn = Kuhn::new();
        let node = Node::new(&kuhn, HoleCards::new_with_rank(0), HoleCards::new_with_rank(1));
        let next_node = node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }
}