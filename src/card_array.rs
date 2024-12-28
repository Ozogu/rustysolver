use crate::card::Card;
use crate::hand_rank::HandRank;
use crate::suit::Suit;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardArray {
    pub ranks: [u8; 14],
    pub suits: [u8; 4],
    pub cards: Vec<Card>,
}

impl CardArray {
    pub fn new() -> Self {
        CardArray {
            ranks: [0; 14],
            suits: [0; 4],
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
        self.ranks[(card.rank - 1) as usize] += 1;
        if card.rank == 14 {
            self.ranks[0] += 1;
        }
        self.suits[card.suit.to_usize()] += 1;
        self.cards.push(card.clone());
    }

    pub fn get_straight_flush(&self) -> HandRank {
        let flush = self.get_flush();
        if flush != HandRank::None {
            let flush_card_array = flush.get_card_array();
            if flush_card_array.get_straight() != HandRank::None {
                return HandRank::StraightFlush(flush_card_array.clone());
            }
        }

        HandRank::None
    }

    pub fn get_flush(&self) -> HandRank {
        
        for suit in self.suits.iter() {
            if *suit >= 5 {
                let mut relevant_cards = CardArray::new();

                for card in self.cards.iter() {
                    if card.suit == Suit::from_u8(*suit) {
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
        let mut bits = 0;
        for rank in self.ranks.iter() {
            if rank > &0 {
                bits = (bits << 1) | rank;
                let straight = 0b11111;
                if bits & straight == straight {
                    let mut relevant_cards = CardArray::new();
                    for card in self.ranks[(rank-1) as usize]..self.ranks[(rank-6) as usize] {
                        relevant_cards.add_card(&Card::new(card as u8, Suit::Hearts));
                    }

                    return HandRank::Straight(relevant_cards);
                }
            }
        }
        
        HandRank::None
    }

    pub fn get_pair_type(&self) -> HandRank {
        let mut pair_type = HandRank::None;
        let mut relevant_cards = CardArray::new();
        for rank in self.ranks {
            if  rank > 1 {
                if pair_type == HandRank::None {
                    pair_type = self.pair_type_from_card_count(rank);
                    self.add_relevant_pair_cards(&mut relevant_cards, rank);
                } else {
                    match pair_type {
                        HandRank::OnePair(_) => {
                            pair_type = self.many_pair_type_from_pair(rank);
                            self.add_relevant_pair_cards(&mut relevant_cards, rank);
                        },
                        HandRank::ThreeOfAKind(_) => {
                            pair_type = self.many_pair_type_from_trips(rank);
                            self.add_relevant_pair_cards(&mut relevant_cards, rank);
                        },
                        _ => panic!("Invalid pair type"),
                    }
    
                    break;
                }
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

    fn add_relevant_pair_cards(&self, relevant_cards: &mut CardArray, rank: u8) {
        for card in self.cards.iter() {
            if card.rank == rank {
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
        assert_eq!(card_array.ranks, [0; 14]);
        assert_eq!(card_array.suits, [0; 4]);
    }

    #[test]
    fn test_add_ace_of_spades() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(14, Suit::Spades));
        assert_eq!(card_array.ranks[13], 1);
        assert_eq!(card_array.ranks[0], 1);
    }

    #[test]
    fn test_add_2_hearths() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Hearts));
        assert_eq!(card_array.ranks[1], 2);
        assert_eq!(card_array.suits[Suit::Hearts.to_usize()], 2);
    }

    #[test]
    fn test_is_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Hearts));
        assert_eq!(card_array.get_flush(), HandRank::Flush(card_array.clone()));
    }

    #[test]
    fn test_is_not_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Spades));
        assert_eq!(card_array.get_flush(), HandRank::None);
    }

    #[test]
    fn test_wheel_straight() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(14, Suit::Spades));
        assert_eq!(card_array.get_straight(), HandRank::Straight(card_array.clone()));
    }

    #[test]
    fn test_not_straight() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(7, Suit::Spades));
        assert_eq!(card_array.get_straight(), HandRank::None);
    }

    #[test]
    fn test_straight_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(6, Suit::Hearts));
        assert_eq!(card_array.get_straight_flush(), HandRank::StraightFlush(card_array.clone()));
    }

    #[test]
    fn test_not_straight_flush() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        card_array.add_card(&Card::new(7, Suit::Hearts));
        assert_eq!(card_array.get_straight_flush(), HandRank::None);
    }

    #[test]
    fn test_pair_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(4, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type(), HandRank::OnePair(card_array.clone()));
    }

    #[test]
    fn test_two_pair_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Spades));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type(), HandRank::TwoPair(card_array.clone()));
    }

    #[test]
    fn test_trips_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(5, Suit::Hearts));
        assert_eq!(card_array.get_pair_type(), HandRank::ThreeOfAKind(card_array.clone()));
    }

    #[test]
    fn test_full_house_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        card_array.add_card(&Card::new(3, Suit::Spades));
        assert_eq!(card_array.get_pair_type(), HandRank::FullHouse(card_array.clone()));
    }

    #[test]
    fn test_four_of_a_kind_detection() {
        let mut card_array = CardArray::new();
        card_array.add_card(&Card::new(2, Suit::Hearts));
        card_array.add_card(&Card::new(2, Suit::Spades));
        card_array.add_card(&Card::new(2, Suit::Clubs));
        card_array.add_card(&Card::new(2, Suit::Diamonds));
        card_array.add_card(&Card::new(3, Suit::Hearts));
        assert_eq!(card_array.get_pair_type(), HandRank::FourOfAKind(card_array.clone()));
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
        assert_eq!(card_array.get_pair_type(), HandRank::FourOfAKind(card_array.clone()));
    }
}