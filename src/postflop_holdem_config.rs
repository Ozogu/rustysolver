use std::collections::HashMap;
use crate::bet::Bet;
use crate::board::Board;
use crate::player::Player;
use crate::range::Range;
use crate::suit::Suit;
use crate::card::Card;

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
                    (Player::IP, Range::new(vec![

                    ])),
                    (Player::OOP, Range::new(vec![

                    ]))
                ]
            ),
            initial_pot: 53.0,
            effective_stack: 74.0,
            flop: Board::from_vec(vec![Card::new(14, Suit::Diamonds), Card::new(11, Suit::Clubs), Card::new(2, Suit::Hearts)]),
            flop_sizes: vec![Bet::P(20)],
            turn_sizes: vec![Bet::P(50)],
            river_sizes: vec![Bet::P(100)],
            default_size: Bet::P(30),
        }
    }
}