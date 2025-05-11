use crate::hole_cards::HoleCards;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Range {
    pub range: HashMap<HoleCards, f64>,
}

pub struct RangeIter<'a> {
    iter: std::collections::hash_map::Iter<'a, HoleCards, f64>,
}

impl<'a> Iterator for RangeIter<'a> {
    type Item = (&'a HoleCards, &'a f64);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}

impl<'a> IntoIterator for &'a Range {
    type Item = (&'a HoleCards, &'a f64);
    type IntoIter = RangeIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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

    pub fn extend(&mut self, other: &Range) {
        for (hole_cards, weight) in &other.range {
            self.range.insert(hole_cards.clone(), *weight);
        }
    }

    pub fn expand_meta_suits(&self) -> Range {
        let mut expanded_range = HashMap::new();

        for (hole_cards, weight) in &self.range {
            let expanded_hole_cards = hole_cards.expand();
            for expanded_hole_card in expanded_hole_cards {
                expanded_range.insert(expanded_hole_card, *weight);
            }
        }

        Range {
            range: expanded_range,
        }
    }

    pub fn iter(&self) -> RangeIter {
        RangeIter {
            iter: self.range.iter(),
        }
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


    #[test]
    fn test_extend() {
        let mut range1 = Range::new(vec![(1.0, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(14, Suit::Diamonds)))]);

        let range2 = Range::new(vec![(0.5, HoleCards::new(&Card::new(13, Suit::Hearts), &Card::new(13, Suit::Diamonds)))]);
        range1.extend(&range2);

        assert_eq!(range1.range.len(), 2);
        assert_eq!(range1.range[&HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(14, Suit::Diamonds))], 1.0);
        assert_eq!(range1.range[&HoleCards::new(&Card::new(13, Suit::Hearts), &Card::new(13, Suit::Diamonds))], 0.5);
    }

    #[test]
    fn test_expand_meta_suits() {
        let range_str = "AhAd:1.0;KhKd:0.5;QQ;AQo:0.2;AKs;72o:0";
        let range = Range::new_from_string(range_str);
        let expanded_range = range.expand_meta_suits();


        let expected_range = Range::new(vec![
            (1.0, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(14, Suit::Diamonds))),

            (0.5, HoleCards::new(&Card::new(13, Suit::Hearts), &Card::new(13, Suit::Diamonds))),

            (1.0, HoleCards::new(&Card::new(12, Suit::Hearts), &Card::new(12, Suit::Diamonds))),
            (1.0, HoleCards::new(&Card::new(12, Suit::Hearts), &Card::new(12, Suit::Clubs))),
            (1.0, HoleCards::new(&Card::new(12, Suit::Hearts), &Card::new(12, Suit::Spades))),
            (1.0, HoleCards::new(&Card::new(12, Suit::Diamonds), &Card::new(12, Suit::Clubs))),
            (1.0, HoleCards::new(&Card::new(12, Suit::Diamonds), &Card::new(12, Suit::Spades))),
            (1.0, HoleCards::new(&Card::new(12, Suit::Clubs), &Card::new(12, Suit::Spades))),

            (0.2, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(12, Suit::Diamonds))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(12, Suit::Clubs))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(12, Suit::Spades))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Diamonds), &Card::new(12, Suit::Clubs))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Diamonds), &Card::new(12, Suit::Spades))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Diamonds), &Card::new(12, Suit::Hearts))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(12, Suit::Diamonds))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(12, Suit::Spades))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(12, Suit::Hearts))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Spades), &Card::new(12, Suit::Diamonds))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Spades), &Card::new(12, Suit::Clubs))),
            (0.2, HoleCards::new(&Card::new(14, Suit::Spades), &Card::new(12, Suit::Hearts))),

            (1.0, HoleCards::new(&Card::new(14, Suit::Hearts), &Card::new(13, Suit::Hearts))),
            (1.0, HoleCards::new(&Card::new(14, Suit::Diamonds), &Card::new(13, Suit::Diamonds))),
            (1.0, HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(13, Suit::Clubs))),
            (1.0, HoleCards::new(&Card::new(14, Suit::Spades), &Card::new(13, Suit::Spades))),
        ]);

        assert_eq!(expanded_range.range.len(), expected_range.range.len());
        for (hole_cards, weight) in expected_range.range {
            assert_eq!(expanded_range.range[&hole_cards], weight);
        }
    }
}
