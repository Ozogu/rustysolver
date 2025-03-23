use crate::hole_cards::HoleCards;

pub struct Range {
    range: Vec<HoleCards>,
}

impl Range {
    pub fn new(range: Vec<HoleCards>) -> Self {
        Range {
            range,
        }
    }

    pub fn range(&self) -> &Vec<HoleCards> {
        &self.range
    }
}