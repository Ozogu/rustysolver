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
use crate::deck::Deck;
use crate::deal::Deal;
use crate::street::Street;

#[derive(Clone, Debug)]
pub struct Node {
    pub reach_prob: HashMap<Player, f64>,
    pub actions: Vec<Action>,
    pub pot: Pot,
    pub history: History,
    pub player: Player,
    pub cards: PlayerCards,
    pub deck: Deck,
}

impl Node {
    pub fn new<G: Game>(game: &G, deal: Deal) -> Node {
        Node {
            actions: game.legal_first_actions(),
            reach_prob: HashMap::from([(Player::IP, 1.0), (Player::OOP, 1.0)]),
            pot: game.initial_pot(),
            history: History::new(),
            player: Player::OOP,
            cards: deal.cards,
            deck: deal.deck,
        }
    }

    pub fn is_terminal<G: Game>(&self, game: &G) -> bool {
        self.history.is_terminal_action() ||
        (self.history.street().to_u8() == game.num_streets() && self.history.is_completing_action())
    }

    pub fn is_street_completing_action(&self) -> bool {
        self.history.is_completing_action()
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

    pub fn board(&self) -> Board {
        self.history.street().board()
    }

    pub fn next_action_node<G: Game>(&self, game: &G, action: Action, action_prob: f64) -> Node {
        let mut next_node: Node = self.clone();
        next_node.history.push_action(action.clone());
        next_node.player = self.player.opponent();
        next_node.reach_prob.insert(self.player(), self.player_reach_prob() * action_prob);
        next_node.actions = game.legal_actions(&next_node.history);
        next_node.pot.update(self.player(), action);
        next_node
    }

    pub fn next_street_node<G: Game>(&self, game: &G, next_street: Street) -> Node {
        let mut next_node: Node = self.clone();
        next_node.history.push_street(next_street);
        next_node.player = Player::OOP;
        next_node.actions = game.legal_actions(&next_node.history);
        next_node
    }

    pub fn info_state(&self) -> InfoState {
        InfoState::new(self.player, self.player_cards(), self.history.clone())
    }

    pub fn log(&self) {
        println!("Player: {:}", self.player);
        println!("Reach prob: {:?}", self.reach_prob);
        println!("Pot: {:?}", self.pot);
        println!("History: {:}", self.history);
        println!("Player cards: {:}", self.player_cards());
        println!("Opponent cards: {:}", self.opponent_cards());
        println!("Deck: {:}", self.deck);
        println!("-----------------");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kuhn::Kuhn;
    use crate::history_node::HistoryNode;
    use crate::bet::Bet;
    use crate::leduc::Leduc;

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
        let next_node = node.next_action_node(&Kuhn::new(), Action::Check, 0.5);
        assert_eq!(next_node.player_reach_prob(), 1.0);
        assert_eq!(next_node.opponent_reach_prob(), 0.5);

        let next_node = next_node.next_action_node(&Kuhn::new(), Action::Check, 0.25);
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
        let next_node = node.next_action_node(&Kuhn::new(), Action::Bet(Bet::P(50)), 0.5);
        assert_eq!(next_node.reach_prob[&Player::OOP], 0.5);
        assert_eq!(next_node.reach_prob[&Player::IP], 1.0);
        assert_eq!(next_node.actions, Kuhn::new().legal_actions(&History::new_from_vec(vec![HistoryNode::Action(Action::Bet(Bet::P(50)))])));
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

        let next_node = node.next_action_node(&Kuhn::new(), Action::Check, 1.0);
        assert_eq!(next_node.player_cards(), ip_cards);
        assert_eq!(next_node.opponent_cards(), oop_cards);
    }

    #[test]
    fn test_next_street_node() {
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)),
            Deck::new_empty()
        );
        let node = Node::new(&Leduc::new(), deal);
        let next_node = node.next_street_node(&Leduc::new(), Street::Flop(Board::new()));
        assert_eq!(next_node.history.street().is_flop(), true);
        assert_eq!(next_node.player, Player::OOP);
        assert_eq!(next_node.actions, Leduc::new().legal_actions(
            &History::new_from_vec(vec![HistoryNode::Street(Street::Flop(Board::new()))])));
    }

    #[test]
    fn test_fold_is_terminal_in_2_street_game() {
        let deal = Deal::new(
            PlayerCards::new(HoleCards::new_with_rank(1), HoleCards::new_with_rank(2)),
            Deck::new_empty()
        );
        let node = Node::new(&Leduc::new(), deal);
        let next_node = node.next_action_node(&Leduc::new(), Action::Fold, 1.0);
        assert_eq!(next_node.is_terminal(&Leduc::new()), true);
    }
}
