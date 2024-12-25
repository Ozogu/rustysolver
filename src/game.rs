use crate::deck::Deck;
use crate::history::History;
use crate::hole_cards::HoleCards;
use crate::action::Action;
use crate::pot::Pot;
use crate::node::Node;
use crate::player_cards::PlayerCards;
use crate::deal::Deal;
use rand::rngs::StdRng;

pub trait Game {
    fn initial_pot(&self) -> Pot;
    fn deck(&self) -> Deck;
    fn get_legal_actions(&self, history: &History) -> Vec<Action>;
    fn player_wins(&self, node: &Node) -> Option<bool>;
    fn num_streets(&self) -> u8;
    fn num_hole_cards(&self) -> u8;
    fn generate_deals(&self) -> Vec<Deal>;

    fn shuffled_cards(&self, rng: &mut StdRng) -> Deck {
        let mut cards = self.deck();
        cards.shuffle(rng);
        cards.reverse();
        cards
    }

    fn deal(&self, rng: &mut StdRng) -> Deal {
        let mut deck = self.shuffled_cards(rng);
        let card1 = deck.draw().unwrap();
        let card2 = deck.draw().unwrap();

        let ip_cards = HoleCards::new_with_rank(card1.rank);
        let oop_cards = HoleCards::new_with_rank(card2.rank);
        let cards = PlayerCards::new(ip_cards, oop_cards);

        Deal::new(cards, deck)
    } 
}
