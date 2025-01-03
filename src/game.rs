use crate::deck::Deck;
use crate::hand_rank::HandRank;
use crate::hand_rank::player_wins;
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
    fn legal_actions(&self, history: &History) -> Vec<Action>;
    fn legal_first_actions(&self) -> Vec<Action>;

    fn num_streets(&self) -> u8;
    fn num_hole_cards(&self) -> u8;

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
    
    fn player_wins(&self, node: &Node) -> Option<bool> {
        let last = node.history.last().unwrap().action();
        match last {
            Action::Fold => Some(true),
            Action::Check | Action::Call => {
                player_wins(
                    node.player_cards(), node.opponent_cards(), node.board())
            }
            _ => panic!("Invalid action: {:?}", last),
        }
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
                
                let deal1 = Deal::new(PlayerCards::new(cards1.clone(), cards2.clone()), deck_clone.clone());
                let deal2 = Deal::new(PlayerCards::new(cards2.clone(), cards1.clone()), deck_clone.clone());

                deals.push(deal1);
                deals.push(deal2);
            }
        }

        deals
    }
}
