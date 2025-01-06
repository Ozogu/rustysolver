use crate::board::Board;
use crate::card::Card;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Street {
    Preflop,
    Flop(Board),
    Turn(Board),
    River(Board),
    None,
}

impl Street {
    pub fn next_street(&self, card: Card) -> Street {
        let mut board = self.board();
        board.push(card);
        match self {
            Street::Preflop => Street::Flop(board),
            Street::Flop(_) => Street::Turn(board),
            Street::Turn(_) => Street::River(board),
            _ => panic!("Cannot advance street"),
        }
    }

    pub fn to_u8(&self) -> u8 {
        match self {
            Street::None => 0,
            Street::Preflop => 1,
            Street::Flop(_) => 2,
            Street::Turn(_) => 3,
            Street::River(_) => 4,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Street::Preflop => "P".to_string(),
            Street::Flop(board) => format!("f{}", board.to_string()),
            Street::Turn(board) => format!("t{}", board.to_string()),
            Street::River(board) => format!("r{}", board.to_string()),
            Street::None => panic!("Cannot convert None street to string"),
        }
    }

    pub fn is_flop(&self) -> bool {
        match self {
            Street::Flop(_) => true,
            _ => false,
        }
    }

    pub fn is_turn(&self) -> bool {
        match self {
            Street::Turn(_) => true,
            _ => false,
        }
    }

    pub fn is_river(&self) -> bool {
        match self {
            Street::River(_) => true,
            _ => false,
        }
    }

    pub fn board(&self) -> Board {
        match self {
            Street::Preflop => Board::new(),
            Street::Flop(board) => board.clone(),
            Street::Turn(board) => board.clone(),
            Street::River(board) => board.clone(),
            Street::None => panic!("Cannot get board for None street"),
        }
    }
}