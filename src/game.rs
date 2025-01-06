use crate::deck::Deck;
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
        let deck = self.deck();

        for c1 in 0..deck.len() {
            for c2 in 0..deck.len() {
                if c1 == c2 { continue; }
                for c3 in 0..deck.len() {
                    if c1 == c3 || c2 == c3 { continue; }
                    for c4 in 0..deck.len() {
                        if c1 == c4 || c2 == c4 || c3 == c4 { continue; }

                        let card1 = deck.get(c1).unwrap();
                        let card2 = deck.get(c2).unwrap();
                        let card3 = deck.get(c3).unwrap();
                        let card4 = deck.get(c4).unwrap();

                        let ip_cards = HoleCards::new(&card1, &card2);
                        let oop_cards = HoleCards::new(&card3, &card4);
                        let cards = PlayerCards::new(ip_cards, oop_cards);


                        let mut card_indexes = vec![c1, c2, c3, c4];
                        card_indexes.sort();

                        let mut deck_clone = deck.clone();
                        for card_index in card_indexes.iter().rev() {
                            deck_clone.remove_index(*card_index);
                        }

                        let deal = Deal::new(cards.clone(), deck_clone.clone());
                        deals.push(deal);
                    }
                }
            }
        }

        deals
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::leduc::Leduc;

    #[test]
    fn test_generate_deals() {
        let game = Leduc::new();
        let deals = game.generate_deals();
        for deal in deals {
            assert_ne!(deal.cards.ip, deal.cards.oop);
            assert_eq!(deal.deck.len(), 2);
        }
    }
}