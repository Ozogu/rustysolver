use crate::board::Board;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Street {
    Preflop,
    Flop(Board),
    Turn(Board),
    River(Board),
    None,
}

impl Street {
    pub fn to_u8(&self) -> u8 {
        match self {
            Street::Preflop => 0,
            Street::Flop(_) => 1,
            Street::Turn(_) => 2,
            Street::River(_) => 3,
            Street::None => 255,
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Street::Preflop => "P".to_string(),
            Street::Flop(_) => "F".to_string(),
            Street::Turn(_) => "T".to_string(),
            Street::River(_) => "R".to_string(),
            Street::None => "N".to_string(),
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
}