use rustysolver::player::Player;
use rustysolver::statistics_visitor::StatisticsVisitor;
use rustysolver::game::Game;
use rustysolver::pot::Pot;
use rustysolver::deck::Deck;
use rustysolver::deal::Deal;
use rustysolver::hole_cards::HoleCards;
use rustysolver::player_cards::PlayerCards;
use rand::rngs::StdRng;
use rustysolver::action::Action;
use rustysolver::bet::Bet;
use rustysolver::card::Card;
use rustysolver::suit::Suit;
use rustysolver::history::History;
use rustysolver::history_node::HistoryNode;
use rustysolver::info_state::InfoState;
use rustysolver::game_tree::GameTree;
use rustysolver::street::Street;
use rustysolver::board::Board;

// OOP card: 2, IP card: 1, Community card: 3
// .
// └── OOP bets 1 (Pot: 3) → EV = 1/3 * 1 + 1/3 * 11/6 + 1/3 * 1/4 = 111/108
//     ├── IP folds (1/3) → OOP +1
//     ├── IP calls (1/3) (Pot: 4)
//     │   └── Community card revealed
//     │       └── OOP bets 1 (Pot: 5) → EV = 1/3 * 2 + 1/3 * 3 + 1/3 * 0.5 = +11/6
//     │           ├── IP folds (1/3) → OOP +2
//     │           ├── IP calls (1/3) → OOP +3
//     │           └── IP raises (1/3) (Pot: 7) → EV = 1/2 * -3 + 1/2 * 4 = +0.5
//     │               ├── OOP folds (1/2) → OOP -3
//     │               └── OOP calls (1/2) → OOP +4
//     └── IP raises (1/3) (Pot: 5) → EV = 1/2 * -2 + 1/2 * 2.5 = 1/4
//         ├── OOP folds (1/2) → OOP -2
//         └── OOP calls (1/2) (Pot: 6)
//             └── Community card revealed
//                 └── OOP bets 1 (Pot: 7) → EV = 1/3 * 3 + 1/3 * 4 + 1/3 * 0.5 = +2.5
//                     ├── IP folds (1/3) → OOP +3
//                     ├── IP calls (1/3) → OOP +4
//                     └── IP raises (1/3) (Pot: 9) → EV = 1/2 * -4 + 1/2 * 5 = +0.5
//                         ├── OOP folds (1/2) → OOP -4
//                         └── OOP calls (1/2) → OOP +5

#[derive(Debug, Clone)]
struct OneStreetGame;

impl OneStreetGame {
    pub fn new() -> Self {
        OneStreetGame
    }
}

impl Game for OneStreetGame {
    fn initial_pot(&self) -> Pot {
        Pot::new(1.0, 1.0)
    }

    fn deck(&self) -> Deck {
        Deck::new_from_cards(vec![
            Card::new(1, Suit::Diamonds),
            Card::new(2, Suit::Diamonds),
            Card::new(3, Suit::Diamonds),
        ])
    }

    fn legal_actions(&self, history: &History) -> Vec<Action> {
        let last = history.last().unwrap_or(&HistoryNode::Action(Action::Check));

        if last.is_street() {
            return vec![Action::Bet(Bet::C(1))];
        }

        match last.action() {
            Action::Check => vec![Action::Check, Action::Bet(Bet::C(1))],
            Action::Bet(_) => vec![Action::Fold, Action::Call, Action::Raise(Bet::C(1))],
            Action::Raise(_) => vec![Action::Fold, Action::Call],
            _ => vec![],
        }
    }

    fn legal_first_actions(&self) -> Vec<Action> {
        vec![Action::Bet(Bet::C(1))]
    }

    fn generate_deals(&self) -> Vec<Deal> {
        let mut deals = Vec::new();
        deals.push(Deal::new(PlayerCards::new(
            HoleCards::new_with_ranks(1, 1),
            HoleCards::new_with_ranks(2, 2)),
            Deck::new_from_cards(vec![Card::new(3, Suit::Diamonds)]),
        ));

        deals
    }

    fn deal(&self, rng: &mut StdRng) -> Deal {
        Deal::new_default()
    }

    fn num_streets(&self) -> u8 {
        2
    }
}


fn one_street_game_tree() -> GameTree<OneStreetGame> {
    let game = OneStreetGame::new();
    let mut tree = GameTree::new(game);
    tree.build();

    tree
}

#[test]
fn test_one_street_game() {
    let tree = one_street_game_tree();
    tree.print_tree();
    let mut visitor = StatisticsVisitor::new(&tree);
    visitor.build();

    let info_state = InfoState::new_empty();
    let ideal_ev = 111.0/108.0;

    debug_assert!((visitor.node_util(&info_state) - ideal_ev).abs() < 1e-6,
        "Strategy EV: {:.4}, Ideal: {:.4}",
        visitor.node_util(&info_state), ideal_ev);
}

#[test]
fn test_b_c_ev() {
    let tree = one_street_game_tree();
    let mut visitor = StatisticsVisitor::new(&tree);
    visitor.build();

    let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(2, 2), History::new_from_vec(vec![
        HistoryNode::Action(Action::Bet(Bet::C(1))),
        HistoryNode::Action(Action::Call),
        HistoryNode::Street(Street::Flop(Board::from_vec(vec![Card::new(3, Suit::Diamonds)]))),
    ]));

    let expected_ev = 11.0/6.0 * (1.0/3.0); // Action utils * reach prob
    debug_assert!((visitor.node_util(&info_state) - expected_ev).abs() < 1e-4,
        "Node EV: {:.4}, Excpected: {:.4}",
        visitor.node_util(&info_state), expected_ev);
}

#[test]
fn test_b_r_c_ev() {
    let tree = one_street_game_tree();
    let mut visitor = StatisticsVisitor::new(&tree);
    visitor.build();

    let info_state = InfoState::new(Player::OOP, HoleCards::new_with_ranks(2, 2), History::new_from_vec(vec![
        HistoryNode::Action(Action::Bet(Bet::C(1))),
        HistoryNode::Action(Action::Raise(Bet::C(1))),
        HistoryNode::Action(Action::Call),
        HistoryNode::Street(Street::Flop(Board::from_vec(vec![Card::new(3, Suit::Diamonds)]))),
    ]));

    let expected_ev = 2.5 * (1.0/3.0) * (1.0/2.0); // Action utils * reach prob
    debug_assert!((visitor.node_util(&info_state) - expected_ev).abs() < 1e-4,
        "Node EV: {:.4}, Excpected: {:.4}",
        visitor.node_util(&info_state), expected_ev);
}