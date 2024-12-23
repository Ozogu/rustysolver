use crate::deck::Deck;
use crate::hole_cards::HoleCards;
use crate::action::Action;
use crate::pot::Pot;
use crate::info_state::InfoState;
use crate::node::Node;
use rand::rngs::StdRng;

pub trait Game {
    fn initial_pot(&self) -> Pot;
    fn deck(&self) -> Deck;
    fn get_legal_actions(&self, info_state: &InfoState) -> Vec<Action>;
    fn player_wins(&self, node: &Node) -> Option<bool>;

    fn shuffled_cards(&self, rng: &mut StdRng) -> Deck {
        let mut cards = self.deck();
        cards.shuffle(rng);
        cards.reverse();
        cards
    }

    fn deal(&self, rng: &mut StdRng) -> (HoleCards, HoleCards, Deck) {
        let mut cards = self.shuffled_cards(rng);
        let card1 = cards.draw().unwrap();
        let card2 = cards.draw().unwrap();

        let ip_cards = HoleCards::new_with_rank(card1.rank);
        let oop_cards = HoleCards::new_with_rank(card2.rank);
    
        (ip_cards, oop_cards, cards)
    } 
}
