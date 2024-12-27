use crate::board::Board;
use crate::hole_cards::HoleCards;
use crate::card_array::CardArray;

#[derive(Debug, PartialEq, Eq)]
pub enum HandRank {
    StraightFlush(CardArray),
    FourOfAKind(CardArray),
    FullHouse(CardArray),
    Flush(CardArray),
    Straight(CardArray),
    ThreeOfAKind(CardArray),
    TwoPair(CardArray),
    OnePair(CardArray),
    HighCard(CardArray),
    None,
}

impl HandRank {
    pub fn to_u8(&self) -> u8 {
        match self {
            HandRank::StraightFlush(_) => 9,
            HandRank::FourOfAKind(_) => 8,
            HandRank::FullHouse(_) => 7,
            HandRank::Flush(_) => 6,
            HandRank::Straight(_) => 5,
            HandRank::ThreeOfAKind(_) => 4,
            HandRank::TwoPair(_) => 3,
            HandRank::OnePair(_) => 2,
            HandRank::HighCard(_) => 1,
            HandRank::None => 0,
        }
    }

    pub fn get_card_array(&self) -> &CardArray {
        match self {
            HandRank::StraightFlush(card_array) => card_array,
            HandRank::FourOfAKind(card_array) => card_array,
            HandRank::FullHouse(card_array) => card_array,
            HandRank::Flush(card_array) => card_array,
            HandRank::Straight(card_array) => card_array,
            HandRank::ThreeOfAKind(card_array) => card_array,
            HandRank::TwoPair(card_array) => card_array,
            HandRank::OnePair(card_array) => card_array,
            HandRank::HighCard(card_array) => card_array,
            HandRank::None => panic!("Invalid card array"),
        }
    }
}

impl PartialOrd for HandRank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let self_rank = self.to_u8();
        let other_rank = other.to_u8();

        if self_rank > other_rank {
            Some(std::cmp::Ordering::Greater)
        } else if self_rank < other_rank {
            Some(std::cmp::Ordering::Less)
        } else {
            let self_card_array = self.get_card_array();
            let other_card_array = other.get_card_array();

            match self {
                HandRank::StraightFlush(_) => compare_straight_flush(self_card_array, other_card_array),
                _ => panic!("Invalid comparsion"),
            }
        }
    }
}        

pub fn player_wins(player: HoleCards, opponent: HoleCards, board: Board) -> Option<bool> {
    let player_rank = calculate_hand_rank(&player, &board);
    let opponent_rank = calculate_hand_rank(&opponent, &board);

    let result = player_rank.partial_cmp(&opponent_rank);
    match result {
        Some(std::cmp::Ordering::Greater) => Some(true),
        Some(std::cmp::Ordering::Less) => Some(false),
        _ => None,
    }
}

fn compare_straight_flush(self_card_array: &CardArray, other_card_array: &CardArray) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Equal)
}

fn calculate_hand_rank(hole_cards: &HoleCards, board: &Board) -> HandRank {
    let card_array = to_card_array(hole_cards, board);

    let is_flush = card_array.is_flush();
    let is_straight = card_array.is_straight();

    if is_flush && is_straight { return HandRank::StraightFlush(card_array); }
    let pair_type = card_array.get_pair_type();
    if pair_type == HandRank::FourOfAKind(card_array.clone()) { return pair_type; }
    else if pair_type == HandRank::FullHouse(card_array.clone()) { return pair_type; } 
    else if is_flush { return HandRank::Flush(card_array); }
    else if is_straight { return HandRank::Straight(card_array); }
    else if pair_type == HandRank::ThreeOfAKind(card_array.clone()) { return pair_type; }
    else if pair_type == HandRank::TwoPair(card_array.clone()) { return pair_type; }
    else if pair_type == HandRank::OnePair(card_array.clone()) { return pair_type; }
    else { return HandRank::HighCard(card_array.clone()); }
}

fn to_card_array(hole_cards: &HoleCards, board: &Board) -> CardArray {
    let mut cards = board.to_vec();
    cards.push(hole_cards.card1.clone());
    cards.push(hole_cards.card2.clone());

    CardArray::from_vec(&cards)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    use crate::card_array;
    use crate::suit::Suit;

    #[test]
    fn test_flush_is_greater_than_straight() {
        assert!(HandRank::Flush(CardArray::new()) > HandRank::Straight(CardArray::new()));
    }

    #[test]
    fn test_trips_is_smaller_than_quad() {
        assert!(HandRank::ThreeOfAKind(CardArray::new()) < HandRank::FourOfAKind(CardArray::new()));
    }

    #[test]
    fn test_quad_board() {
        let mut card_array1 = CardArray::from_vec(&vec![
                Card::new(4, Suit::Clubs),
                Card::new(3, Suit::Clubs),
                Card::new(2, Suit::Clubs),
                Card::new(2, Suit::Diamonds),
                Card::new(2, Suit::Spades),
                Card::new(2, Suit::Hearts),
            ]
        );

        let card_array2 = card_array1.clone();
        card_array1.ranks[0] = 3;
        assert!(HandRank::FourOfAKind(card_array1) > HandRank::FourOfAKind(card_array2));

    }
}