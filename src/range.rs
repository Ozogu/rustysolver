use crate::hole_cards::HoleCards;

pub struct Range {
    pub range: Vec<(f64, HoleCards)>,
}

impl Range {

    pub fn new_pure_range(range: Vec<HoleCards>) -> Self {
        let mut pure_range = Vec::new();
        for hole_cards in range {
            pure_range.push((1.0, hole_cards));
        }

        Range {
            range: pure_range,
        }
    }

    pub fn new(range: Vec<(f64, HoleCards)>) -> Self {
        Range {
            range,
        }
    }
}