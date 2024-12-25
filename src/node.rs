use std::collections::HashMap;
use crate::hole_cards::HoleCards;
use crate::game::Game;
use crate::player::Player;
use crate::info_state::InfoState;
use crate::action::Action;
use crate::player_cards::PlayerCards;
use crate::pot::Pot;
use crate::history::History;
use crate::board::Board;
use crate::street::Street;
use crate::deck::Deck;
use crate::deal::Deal;

#[derive(Clone, Debug)]
pub struct Node {
    pub reach_prob: HashMap<Player, f64>,
    pub actions: Vec<Action>,
    pub pot: Pot,
    pub history: History,
    pub player: Player,
    pub cards: PlayerCards,
    pub board: Board,
    pub street: Street,
    pub deck: Deck,
}

impl Node {
    pub fn new<G: Game>(game: &G, deal: Deal) -> Node {
        Node {
            actions: game.get_legal_actions(&History::new()),
            reach_prob: HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]),
            pot: game.initial_pot(),
            history: History::new(),
            player: Player::OOP,
            cards: deal.cards,
            board: Board::new(),
            street: Street::Preflop,
            deck: deal.deck,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.history.is_terminal()
    }
    
    pub fn player(&self) -> Player {
        self.player
    }

    pub fn player_reach_prob(&self) -> f64 {
        self.reach_prob[&self.player()]
    }

    pub fn opponent_reach_prob(&self) -> f64 {
        self.reach_prob[&self.player().opponent()]
    }

    pub fn player_cards(&self) -> HoleCards {
        self.cards.get(self.player)
    }

    pub fn opponent_cards(&self) -> HoleCards {
        self.cards.get(self.player.opponent())
    }

    pub fn zero_utils(&self) -> Vec<f64> {
        vec![0.0; self.actions.len()]
    }

    pub fn next_node<G: Game>(&self, game: &G, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.history.push(action);
        next_node.player = self.player.opponent();
        next_node.reach_prob.insert(self.player(), self.player_reach_prob() * action_prob);
        next_node.actions = game.get_legal_actions(&next_node.history);
        next_node.pot.update(self.player(), action);
        next_node
    }

    pub fn info_state(&self) -> InfoState {
        InfoState::new(self.player, self.player_cards(), self.history.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kuhn::Kuhn;

    #[test]
    fn test_new() {
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)),
            Deck::new_empty()
        );
        let node = Node::new(&Kuhn::new(), deal);
        assert_eq!(node.reach_prob[&Player::IP], 1.0);
        assert_eq!(node.reach_prob[&Player::OOP], 1.0);
    }

    #[test]
    fn test_reach_prob() {
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)),
            Deck::new_empty()
        );
        let node = Node::new(&Kuhn::new(), deal);
        assert_eq!(node.player_reach_prob(), 1.0);
        assert_eq!(node.opponent_reach_prob(), 1.0);

        // in next node the previous player's (opponent) reach prob is multiplied by the action prob
        let next_node = node.next_node(&Kuhn::new(), Action::Check, 0.5);
        assert_eq!(next_node.player_reach_prob(), 1.0);
        assert_eq!(next_node.opponent_reach_prob(), 0.5);
        
        let next_node = next_node.next_node(&Kuhn::new(), Action::Check, 0.25);
        assert_eq!(next_node.opponent_reach_prob(), 0.25);
        assert_eq!(next_node.player_reach_prob(), 0.5);
    }

    #[test]
    fn test_next_node() {
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)),
            Deck::new_empty()
        );
        let node = Node::new(&Kuhn::new(), deal);
        let next_node = node.next_node(&Kuhn::new(), Action::Bet(50), 0.5);
        assert_eq!(next_node.reach_prob[&Player::OOP], 0.5);
        assert_eq!(next_node.reach_prob[&Player::IP], 1.0);
        assert_eq!(next_node.actions, Kuhn::new().get_legal_actions(&History::new_from_vec(vec![Action::Bet(50)])));
        assert_eq!(next_node.pot.total(), node.pot.total() + 1.0);
        assert_eq!(next_node.pot.contributions(), HashMap::from([(Player::IP, 1.0), (Player::OOP, 2.0)]));
    }

    #[test]
    fn test_player_cards() {
        let ip_cards = HoleCards::new_with_rank(1);
        let oop_cards = HoleCards::new_with_rank(2);
        let deal = Deal::new(PlayerCards::new(ip_cards.clone(), oop_cards.clone()), Deck::new_empty());
        let node = Node::new(&Kuhn::new(), deal);
        assert_eq!(node.player_cards(), oop_cards);
        assert_eq!(node.opponent_cards(), ip_cards);

        let next_node = node.next_node(&Kuhn::new(), Action::Check, 1.0);
        assert_eq!(next_node.player_cards(), ip_cards);
        assert_eq!(next_node.opponent_cards(), oop_cards);
    }
}
