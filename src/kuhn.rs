use crate::action::Action;
use crate::history::History;
use crate::node::Node;
use crate::deck::Deck;
use crate::card::Card;
use crate::suit::Suit;
use crate::pot::Pot;
use crate::game::Game;
use crate::player_cards::PlayerCards;
use crate::hole_cards::HoleCards;
use crate::deal::Deal;

#[derive(Clone, Debug)]
pub struct Kuhn {}

impl Kuhn {
    pub fn new() -> Self {
        Kuhn {}
    }
}

impl Game for Kuhn {
    fn initial_pot(&self) -> Pot {
        Pot::new(1.0, 1.0)
    }

    fn num_streets(&self) -> u8 {
        1
    }

    fn num_hole_cards(&self) -> u8 {
        1
    }

    fn deck(&self) -> Deck {
        Deck::new_from_cards(vec![
            Card::new(0, Suit::Diamonds),
            Card::new(1, Suit::Diamonds),
            Card::new(2, Suit::Diamonds),
        ])
    }

    fn generate_deals(&self) -> Vec<Deal> {
        let mut deals = Vec::new();
        let mut deck = self.deck();
        
        for _ in 0..deck.len() {
            let card = deck.draw().unwrap().rank;
            let cards1 = HoleCards::new_with_rank(card);
            let mut deck_clone = deck.clone();
            for _ in 0..deck_clone.len() {
                let card = deck_clone.draw().unwrap().rank;
                let cards2 = HoleCards::new_with_rank(card);
                
                let deal1 = Deal::new(PlayerCards::new(cards1.clone(), cards2.clone()), deck.clone());
                let deal2 = Deal::new(PlayerCards::new(cards2.clone(), cards1.clone()), deck.clone());

                deals.push(deal1);
                deals.push(deal2);
            }
        }

        deals
    }

    fn get_legal_actions(&self, history: &History) -> Vec<Action> {
        // At root there will be no history
        let last = history.last().unwrap_or(&Action::Check);
        match last {
            Action::Check => vec![Action::Check, Action::Bet(50)],
            Action::Bet(50) => vec![Action::Call, Action::Fold],
            _ => vec![],
        }
    }

    fn player_wins(&self, node: &Node) -> Option<bool> {
        let last = node.history.last().unwrap();
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
    use super::*;
    use crate::player::Player;
    use crate::player_cards::PlayerCards;
    use crate::hole_cards::HoleCards;

    #[test]
    fn test_legal_actions_at_root() {
        let kuhn = Kuhn::new();
        let actions = kuhn.get_legal_actions(&History::new());
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_check() {
        let kuhn = Kuhn::new();
        let history = History::new_from_vec(vec![Action::Check]);
        let actions = kuhn.get_legal_actions(&history);
        assert_eq!(actions, vec![Action::Check, Action::Bet(50)]);
    }

    #[test]
    fn test_legal_actions_after_bet() {
        let kuhn = Kuhn::new();
        let history = History::new_from_vec(vec![Action::Bet(50)]);
        let actions = kuhn.get_legal_actions(&history);
        assert_eq!(actions, vec![Action::Call, Action::Fold]);
    }

    #[test]
    fn test_player_wins_xx() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(0), HoleCards::new_with_rank(1)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Check, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbf() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(0), HoleCards::new_with_rank(1)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::IP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbc() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(0), HoleCards::new_with_rank(1)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::IP, Some(false)));
    }

    #[test]
    fn test_player_wins_bf() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(0), HoleCards::new_with_rank(1)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_bc() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(0), HoleCards::new_with_rank(1)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_node(&kuhn, Action::Bet(50), 1.0);
        let next_node = next_node.next_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player(), kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }
}