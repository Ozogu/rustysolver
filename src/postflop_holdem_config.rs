use std::collections::HashMap;
use crate::bet::Bet;
use crate::board::Board;
use crate::player::Player;
use crate::range::Range;
use crate::suit::Suit;
use crate::card::Card;
use crate::hole_cards::HoleCards;

#[derive(Clone, Debug)]
pub struct PostflopHoldemConfig {
    pub player_range: HashMap<Player, Range>,
    pub flop: Board,
    pub initial_pot: f64,
    pub effective_stack: f64,
    pub flop_sizes: Vec<Bet>,
    pub turn_sizes: Vec<Bet>,
    pub river_sizes: Vec<Bet>,
    pub default_size: Bet,
}

impl PostflopHoldemConfig {
    pub fn new_default() -> Self {
        PostflopHoldemConfig {
            player_range: HashMap::from(
                [
                    (Player::IP, Range::new_pure_range(
                        vec![
                            HoleCards::new_with_rank(14),
                            HoleCards::new_with_rank(12),
                        ]
                    )),
                    (Player::OOP, Range::new_pure_range(
                        vec![
                            HoleCards::new_with_rank(13),
                        ]
                    )),
                ]
            ),
            initial_pot: 53.0,
            effective_stack: 74.0,
            flop: Board::from_vec(vec![Card::new(14, Suit::Diamonds), Card::new(11, Suit::Clubs), Card::new(2, Suit::Hearts)]),
            flop_sizes: vec![Bet::P(25)],
            turn_sizes: vec![Bet::P(125)],
            river_sizes: vec![Bet::P(200)],
            default_size: Bet::P(100),
        }
    }

    pub fn oop_range(&self) -> Range {
        self.player_range.get(&Player::OOP).unwrap().clone()
    }

    pub fn ip_range(&self) -> Range {
        self.player_range.get(&Player::IP).unwrap().clone()
    }
}