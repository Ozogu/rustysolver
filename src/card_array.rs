use crate::card::Card;
use crate::hand_rank::HandRank;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardArray {
    pub rank_counts: [u8; 14],
    pub suit_counts: [u8; 4],
    pub cards: Vec<Card>,
}

impl CardArray {
    pub fn new() -> Self {
        CardArray {
            rank_counts: [0; 14],
            suit_counts: [0; 4],
            cards: Vec::new(),
        }
    }

    pub fn from_vec(cards: &Vec<Card>) -> Self {
        let mut card_array = CardArray::new();
        for card in cards {
            card_array.add_card(card);
        }

        card_array
    }

    pub fn add_card(&mut self, card: &Card) {
        self.rank_counts[(card.rank - 1) as usize] += 1;
        if card.rank == 14 {
            self.rank_counts[0] += 1;
        }
        self.suit_counts[card.suit.to_usize()] += 1;
        self.cards.push(card.clone());
    }

    pub fn get_straight_flush(&self) -> HandRank {
        let flush = self.get_flush();
        if flush.is_flush() {
            let flush_card_array = flush.get_card_array();
            if flush_card_array.get_straight().is_straight() {
                return HandRank::StraightFlush(flush_card_array.clone());
            }
        }

        HandRank::None
    }

    pub fn get_flush(&self) -> HandRank {
        
        for (suit, count) in self.suit_counts.iter().enumerate() {
            if *count >= 5 {
                let mut relevant_cards = CardArray::new();

                for card in self.cards.iter() {
                    if card.suit.to_usize() == suit {
                        relevant_cards.add_card(card);
                    }
                }

                relevant_cards.cards.sort_by(|a, b| b.rank.cmp(&a.rank));
                return HandRank::Flush(relevant_cards);
            }
        }

        HandRank::None
    }

    pub fn get_straight(&self) -> HandRank {
        let mut sum = 0;
        for (i, count) in self.rank_counts.iter().enumerate() {
            if count > &0 {
                sum += 1;
                if sum == 5 {
                    let mut relevant_cards = CardArray::new();
                    for card in i-4..i {
                        relevant_cards.add_card(&self.cards[card]);
                    }

                    return HandRank::Straight(relevant_cards);
                }
            } else {
                sum = 0;
            }
        }
        
        HandRank::None
    }

    pub fn get_pair_type(&self) -> HandRank {
        let mut pair_type = HandRank::None;
        let mut relevant_cards = CardArray::new();
        let mut counts = Vec::new();
        for count in self.rank_counts.iter() { if count > &1 { counts.push(count); } }
        counts.sort_by(|a, b| b.cmp(a));

        for count in counts.iter() {
            if pair_type == HandRank::None {
                pair_type = self.pair_type_from_card_count(**count);
                self.add_relevant_pair_cards(&mut relevant_cards, **count);
                if pair_type.is_four_of_a_kind() { break; }
            } else {
                match pair_type {
                    HandRank::OnePair(_) => {
                        pair_type = self.many_pair_type_from_pair(**count);
                        self.add_relevant_pair_cards(&mut relevant_cards, **count);
                    },
                    HandRank::ThreeOfAKind(_) => {
                        pair_type = self.many_pair_type_from_trips(**count);
                        self.add_relevant_pair_cards(&mut relevant_cards, **count);
                    },
                    _ => panic!("Invalid pair type"),
                }

                break;
            }
        }
        
        if pair_type != HandRank::None {
            self.fill_relevant_cards(&mut relevant_cards);
            return pair_type;
        }
    
        pair_type
    }

    pub fn get_high_card(&self) -> HandRank {
        let mut relevant_cards = CardArray::new();
        self.fill_relevant_cards(&mut relevant_cards);

        HandRank::HighCard(relevant_cards)
    }
    
    fn pair_type_from_card_count(&self, card_count: u8) -> HandRank {
        match card_count {
            2 => HandRank::OnePair(self.clone()),
            3 => HandRank::ThreeOfAKind(self.clone()),
            4 => HandRank::FourOfAKind(self.clone()),
            _ => panic!("Invalid pair type"),
        }
    }

    fn many_pair_type_from_pair(&self, card_count: u8) -> HandRank {
        match card_count {
            2 => HandRank::TwoPair(self.clone()),
            3 => HandRank::FullHouse(self.clone()),
            _ => panic!("Invalid pair type"),
        }
    }

    fn many_pair_type_from_trips(&self, card_count: u8) -> HandRank {
        match card_count {
            2 => HandRank::FullHouse(self.clone()),
            _ => panic!("Invalid pair type"),
        }
    }

    fn add_relevant_pair_cards(&self, relevant_cards: &mut CardArray, count: u8) {
        for card in self.cards.iter() {
            if card.rank == count {
                relevant_cards.add_card(&card);
            }
        }
    }

    fn inner_join(&self, other: &CardArray) -> CardArray {
        let mut joined = CardArray::new();
        for card in self.cards.iter() {
            if other.cards.contains(card) {
                joined.add_card(card);
            }
        }

        joined
    }

    fn fill_relevant_cards(&self, relevant_cards: &mut CardArray) {
        let mut inner_joined = self.inner_join(&relevant_cards);
        inner_joined.cards.sort_by(|a, b| b.rank.cmp(&a.rank));

        let fill_num = 5 - relevant_cards.cards.len();
        inner_joined.cards.iter().take(fill_num)
            .for_each(|card| relevant_cards.add_card(card));
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::suit::Suit;

    #[test]
    fn test_card_array() {
        let card_array = CardArray::new();
        assert_eq!(card_array.rank_counts, [0; 14]);
        assert_eq!(card_array.suit_counts, [0; 4]);
    }

    #[test]
    fn test_add_ace_of_spades() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(14, Suit::Spades));
        assert_eq!(card_array.rank_counts[13], 1);
        assert_eq!(card_array.rank_counts[0], 1);
    }

    #[test]
    fn test_add_2_hearths() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Hearts));
        assert_eq!(card_array.rank_counts[1], 2);
        assert_eq!(card_array.suit_counts[Suit::Hearts.to_usize()], 2);
    }

    #[test]
    fn test_is_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Hearts));
        assert_eq!(card_array.get_flush().is_flush(), true);
    }

    #[test]
    fn test_is_not_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Spades));
        assert_eq!(card_array.get_flush().is_flush(), false);
    }

    #[test]
    fn test_wheel_straight() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(14, Suit::Spades));
        assert_eq!(card_array.get_straight().is_straight(), true);
    }

    #[test]
    fn test_not_straight() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(7, Suit::Spades));
        assert_eq!(card_array.get_straight().is_straight(), false);
    }

    #[test]
    fn test_straight_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Hearts));
        assert_eq!(card_array.get_straight_flush().is_straight_flush(), true);
    }

    #[test]
    fn test_not_straight_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(7, Suit::Hearts));
        assert_eq!(card_array.get_straight_flush().is_straight_flush(), false);
    }

    #[test]
    fn test_pair_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type().is_one_pair(), true);
    }

    #[test]
    fn test_two_pair_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Spades));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type().is_two_pair(), true);
    }

    #[test]
    fn test_trips_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type().is_three_of_a_kind(), true);
    }

    #[test]
    fn test_full_house_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Spades));
        assert_eq!(card_array.get_pair_type().is_full_house(), true);
    }

    #[test]
    fn test_four_of_a_kind_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(2, Suit::Diamonds));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        assert_eq!(card_array.get_pair_type().is_four_of_a_kind(), true);
    }

    #[test]
    fn test_four_of_a_kind_detection_from_quad_trip_board() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(3, Suit::Diamonds));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        assert_eq!(card_array.get_pair_type().is_four_of_a_kind(), true);
    }

    #[test]
    fn test_four_of_a_kind_detection_from_quad_trip_board_reversed() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Diamonds));
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        assert_eq!(card_array.get_pair_type().is_four_of_a_kind(), true);
    }
}