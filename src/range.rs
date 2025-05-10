use crate::hole_cards::HoleCards;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Range {
    pub range: HashMap<HoleCards, f64>,
}

impl Range {
    pub fn new_pure_range(range: Vec<HoleCards>) -> Self {
        let mut range_map = HashMap::new();
        for hole_cards in range {
            range_map.insert(hole_cards, 1.0);
        }

        Range {
            range: range_map,
        }
    }

    pub fn new(range: Vec<(f64, HoleCards)>) -> Self {
        let mut range_map = HashMap::new();
        for (weight, hole_cards) in range {
            range_map.insert(hole_cards, weight);
        }

        Range {
            range: range_map,
        }
    }

    pub fn new_from_string(range_str: &str) -> Self {
        let mut range = HashMap::new();
        let parts: Vec<&str> = range_str.split(';').collect();

        for part in parts {
            let mut split = part.split(':');
            let hole_cards_str = split.next().unwrap();
            let weight_str = split.next();

            let hole_cards = HoleCards::new_from_string(hole_cards_str);
            let weight = match weight_str {
                Some(w) => w.parse::<f64>().unwrap(),
                None => 1.0,
            };

            if weight == 0.0 {
                continue;
            }

            range.insert(hole_cards, weight);
        }

        Range { range }
    }
}
#[cfg(test)]
mod tests {
    use crate::card::Card;
    use crate::suit::Suit;
    use super::*;

    #[test]
    fn test_new_from_string() {
        let range_str = "AhAd:1.0;KhKd:0.5;QQ;AQo:0.2;AKs;72o:0";
        let range = Range::new_from_string(range_str);

        assert_eq!(range.range.len(), 5);
        assert_eq!(range.range[&HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(14, Suit::Diamonds))], 1.0);
        assert_eq!(range.range[&HoleCards::new(&Card::new(13, Suit::Hearts), &Card::new(13, Suit::Diamonds))], 0.5);
        assert_eq!(range.range[&HoleCards::new(&Card::new(12, Suit::Offsuit), &Card::new(12, Suit::Offsuit))], 1.0);
        assert_eq!(range.range[&HoleCards::new(&Card::new(14, Suit::Offsuit), &Card::new(12, Suit::Offsuit))], 0.2);
        assert_eq!(range.range[&HoleCards::new(&Card::new(14, Suit::Suited), &Card::new(13, Suit::Suited))], 1.0);
    }
}
