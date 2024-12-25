use crate::deck::Deck;
use crate::history::History;
use crate::hole_cards::HoleCards;
use crate::action::Action;
use crate::pot::Pot;
use crate::node::Node;
use crate::player_cards::PlayerCards;
use rand::rngs::StdRng;

pub trait Game {
    fn initial_pot(&self) -> Pot;
    fn deck(&self) -> Deck;
    fn get_legal_actions(&self, history: &History) -> Vec<Action>;
    fn player_wins(&self, node: &Node) -> Option<bool>;
    fn num_streets(&self) -> u8;
    fn num_hole_cards(&self) -> u8;
    fn generate_roots(&self) -> Vec<PlayerCards>;

    fn shuffled_cards(&self, rng: &mut StdRng) -> Deck {
        let mut cards = self.deck();
        cards.shuffle(rng);
        cards.reverse();
        cards
    }

    fn deal(&self, rng: &mut StdRng) -> (PlayerCards, Deck) {
        let mut cards = self.shuffled_cards(rng);
        let card1 = cards.draw().unwrap();
        let card2 = cards.draw().unwrap();

        let ip_cards = HoleCards::new_with_rank(card1.rank);
        let oop_cards = HoleCards::new_with_rank(card2.rank);

        (PlayerCards::new(ip_cards, oop_cards), cards)
    } 
}
