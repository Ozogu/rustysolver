use crate::action::Action;
use crate::history::History;
use crate::deck::Deck;
use crate::card::Card;
use crate::suit::Suit;
use crate::pot::Pot;
use crate::game::Game;
use crate::history_node::HistoryNode;
use crate::bet::Bet;
use crate::deal::Deal;
use crate::player_cards::PlayerCards;
use crate::hole_cards::HoleCards;
use rand::rngs::StdRng;

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

    fn deck(&self) -> Deck {
        Deck::new_from_cards(vec![
            Card::new(1, Suit::Diamonds),
            Card::new(2, Suit::Diamonds),
            Card::new(3, Suit::Diamonds),
        ])
    }

    fn legal_actions(&self, history: &History) -> Vec<Action> {
        // At root there will be no history
        let last = history.last().unwrap_or(&HistoryNode::Action(Action::Check)).action();
        match last {
            Action::Check => vec![Action::Check, Action::Bet(Bet::P(50))],
            Action::Bet(Bet::P(50)) => vec![Action::Fold, Action::Call],
            _ => vec![],
        }
    }

    fn legal_first_actions(&self) -> Vec<Action> {
        self.legal_actions(&History::new())
    }

    fn generate_deals(&self) -> Vec<Deal> {
        let mut deals = Vec::new();
        let mut deck = self.deck();

        for _ in 0..deck.len() {
            let card = deck.draw().unwrap().rank;
            let cards1 = HoleCards::new_with_ranks(card, card);
            let mut deck_clone = deck.clone();
            for _ in 0..deck_clone.len() {
                let card = deck_clone.draw().unwrap().rank;
                let cards2 = HoleCards::new_with_ranks(card, card);
                let weights = (1.0, 1.0);

                let deal1 = Deal::new(PlayerCards::new(cards1.clone(), cards2.clone()), deck_clone.clone(), weights);
                let deal2 = Deal::new(PlayerCards::new(cards2.clone(), cards1.clone()), deck_clone.clone(), weights);

                deals.push(deal1);
                deals.push(deal2);
            }
        }

        deals
    }

    fn deal(&self, rng: &mut StdRng) -> Deal {
        let mut deck = self.shuffled_cards(rng);
        let card1 = deck.draw().unwrap();
        let card2 = deck.draw().unwrap();

        let ip_cards = HoleCards::new_with_ranks(card1.rank, card1.rank);
        let oop_cards = HoleCards::new_with_ranks(card2.rank, card2.rank);
        let cards = PlayerCards::new(ip_cards, oop_cards);

        Deal::new(cards, deck, (1.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::player::Player;
    use crate::player_cards::PlayerCards;
    use crate::hole_cards::HoleCards;
    use crate::history_node::HistoryNode;
    use crate::action::Action;
    use crate::deal::Deal;
    use crate::node::Node;

    #[test]
    fn test_legal_actions_at_root() {
        let kuhn = Kuhn::new();
        let actions = kuhn.legal_actions(&History::new());
        assert_eq!(actions, vec![Action::Check, Action::Bet(Bet::P(50))]);
    }

    #[test]
    fn test_legal_actions_after_check() {
        let kuhn = Kuhn::new();
        let history = History::new_from_vec(vec![HistoryNode::Action(Action::Check)]);
        let actions = kuhn.legal_actions(&history);
        assert_eq!(actions, vec![Action::Check, Action::Bet(Bet::P(50))]);
    }

    #[test]
    fn test_legal_actions_after_bet() {
        let kuhn = Kuhn::new();
        let history = History::new_from_vec(vec![HistoryNode::Action(Action::Bet(Bet::P(50)))]);
        let actions = kuhn.legal_actions(&history);
        assert_eq!(actions, vec![Action::Fold, Action::Call]);
    }

    #[test]
    fn test_player_wins_xx() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_ranks(1, 1), HoleCards::new_with_ranks(2, 2)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_action_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Check, 1.0);
        assert_eq!((next_node.player, kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbf() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_ranks(1, 1), HoleCards::new_with_ranks(2, 2)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_action_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Bet(Bet::P(50)), 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player, kuhn.player_wins(&next_node)), (Player::IP, Some(true)));
    }

    #[test]
    fn test_player_wins_xbc() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_ranks(1, 1), HoleCards::new_with_ranks(2, 2)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_action_node(&kuhn, Action::Check, 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Bet(Bet::P(50)), 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player, kuhn.player_wins(&next_node)), (Player::IP, Some(false)));
    }

    #[test]
    fn test_player_wins_bf() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_ranks(1, 1), HoleCards::new_with_ranks(2, 2)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_action_node(&kuhn, Action::Bet(Bet::P(50)), 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Fold, 1.0);
        assert_eq!((next_node.player, kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }

    #[test]
    fn test_player_wins_bc() {
        let kuhn = Kuhn::new();
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_ranks(1, 1), HoleCards::new_with_ranks(2, 2)),
            Deck::new_empty()
        );
        let node = Node::new(&kuhn, deal);
        let next_node = node.next_action_node(&kuhn, Action::Bet(Bet::P(50)), 1.0);
        let next_node = next_node.next_action_node(&kuhn, Action::Call, 1.0);
        assert_eq!((next_node.player, kuhn.player_wins(&next_node)), (Player::OOP, Some(true)));
    }
}