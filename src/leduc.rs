use crate::deck::Deck;
use crate::game::Game;
use crate::history::History;
use crate::hole_cards::HoleCards;
use crate::suit::Suit;
use crate::card::Card;
use crate::pot::Pot;
use crate::action::Action;
use crate::history_node::HistoryNode;
use crate::bet::Bet;
use crate::deal::Deal;
use crate::player_cards::PlayerCards;
use rand::rngs::StdRng;

#[derive(Clone, Debug)]
pub struct Leduc {}

impl Leduc {
    pub fn new() -> Self {
        Leduc {}
    }
}

impl Game for Leduc {
    fn initial_pot(&self) -> Pot {
        Pot::new(1.0, 1.0)
    }

    fn deck(&self) -> crate::deck::Deck {
        Deck::new_from_cards(
            vec![
                Card::new(1, Suit::Diamonds),
                Card::new(1, Suit::Diamonds),
                Card::new(2, Suit::Diamonds),
                Card::new(2, Suit::Diamonds),
                Card::new(3, Suit::Diamonds),
                Card::new(3, Suit::Diamonds),
            ]
        )
    }

    fn num_streets(&self) -> u8 {
        2
    }

    fn legal_actions(&self, history: &History) -> Vec<Action> {
        let last = history.last().unwrap_or(&HistoryNode::Action(Action::Check));
        let default = vec![Action::Check, Action::Bet(Bet::C(2)), Action::Bet(Bet::C(4))];

        if last.is_street() {
            return default;
        }

        match last.action() {
            Action::Check => default,
            Action::Bet(_) => vec![Action::Fold, Action::Call, Action::Raise(Bet::C(2)), Action::Raise(Bet::C(4))],
            Action::Raise(_) => vec![Action::Fold, Action::Call],
            _ => vec![],
        }
    }

    fn legal_first_actions(&self) -> Vec<Action> {
        self.legal_actions(&History::new())
    }

    fn deal(&self, rng: &mut StdRng) -> Deal {
        let mut deck = self.shuffled_cards(rng);
        let card1 = deck.draw().unwrap();
        let card2 = deck.draw().unwrap();
        let card3 = deck.draw().unwrap();
        let card4 = deck.draw().unwrap();

        let ip_cards = HoleCards::new(&card1, &card2);
        let oop_cards = HoleCards::new(&card3, &card4);
        let cards = PlayerCards::new(ip_cards, oop_cards);

        Deal::new(cards, deck, (1.0, 1.0), History::new())
    }
}