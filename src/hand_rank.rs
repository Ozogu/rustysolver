use crate::board::Board;
use crate::hole_cards::HoleCards;
use crate::card_array::{self, CardArray};
use std::fmt;

#[derive(Debug)]
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

    pub fn set_card_array(&mut self, arr: CardArray) {
        match self {
            HandRank::StraightFlush(card_array) => *card_array = arr,
            HandRank::FourOfAKind(card_array) => *card_array = arr,
            HandRank::FullHouse(card_array) => *card_array = arr,
            HandRank::Flush(card_array) => *card_array = arr,
            HandRank::Straight(card_array) => *card_array = arr,
            HandRank::ThreeOfAKind(card_array) => *card_array = arr,
            HandRank::TwoPair(card_array) => *card_array = arr,
            HandRank::OnePair(card_array) => *card_array = arr,
            HandRank::HighCard(card_array) => *card_array = arr,
            HandRank::None => panic!("Invalid card array"),
        }
    }

    pub fn is_straight_flush(&self) -> bool { matches!(self, HandRank::StraightFlush(_)) }
    pub fn is_four_of_a_kind(&self) -> bool { matches!(self, HandRank::FourOfAKind(_)) }
    pub fn is_full_house(&self) -> bool { matches!(self, HandRank::FullHouse(_)) }
    pub fn is_flush(&self) -> bool { matches!(self, HandRank::Flush(_)) }
    pub fn is_straight(&self) -> bool { matches!(self, HandRank::Straight(_)) }
    pub fn is_three_of_a_kind(&self) -> bool { matches!(self, HandRank::ThreeOfAKind(_)) }
    pub fn is_two_pair(&self) -> bool { matches!(self, HandRank::TwoPair(_)) }
    pub fn is_one_pair(&self) -> bool { matches!(self, HandRank::OnePair(_)) }
    pub fn is_high_card(&self) -> bool { matches!(self, HandRank::HighCard(_)) }
    pub fn is_none(&self) -> bool { matches!(self, HandRank::None) }
}

impl PartialEq for HandRank {
    fn eq(&self, other: &Self) -> bool {
        self.to_u8() == other.to_u8()
    }
}

impl Eq for HandRank {}

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

            self_card_array.partial_cmp(other_card_array)
        }
    }
}

impl fmt::Display for HandRank {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HandRank::StraightFlush(card_array) => write!(f, "Straight flush: {:}", card_array),
            HandRank::FourOfAKind(card_array) => write!(f, "Four of a kind: {:}", card_array),
            HandRank::FullHouse(card_array) => write!(f, "Full house: {:}", card_array),
            HandRank::Flush(card_array) => write!(f, "Flush: {:}", card_array),
            HandRank::Straight(card_array) => write!(f, "Straight: {:}", card_array),
            HandRank::ThreeOfAKind(card_array) => write!(f, "Three of a kind: {:}", card_array),
            HandRank::TwoPair(card_array) => write!(f, "Two pair: {:}", card_array),
            HandRank::OnePair(card_array) => write!(f, "One pair: {:}", card_array),
            HandRank::HighCard(card_array) => write!(f, "High card: {:}", card_array),
            HandRank::None => write!(f, "None"),
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

fn calculate_hand_rank(hole_cards: &HoleCards, board: &Board) -> HandRank {
    let mut card_array = CardArray::new();
    card_array.add_card(&hole_cards.card1);
    card_array.add_card(&hole_cards.card2);
    for card    in &board.cards {
        card_array.add_card(card);
    }

    if let straight_flush @ HandRank::StraightFlush(_) = card_array.get_straight_flush() { return straight_flush; }
    let pair_type = card_array.get_pair_type();
    if pair_type.is_four_of_a_kind() { return pair_type; }
    else if pair_type.is_full_house() { return pair_type; }
    else if let flush @ HandRank::Flush(_) = card_array.get_flush() { return flush; }
    else if let straight @ HandRank::Straight(_) = card_array.get_straight() { return straight; }
    else if pair_type.is_three_of_a_kind() { return pair_type; }
    else if pair_type.is_two_pair() { return pair_type; }
    else if pair_type.is_one_pair() { return pair_type; }
    else { return card_array.get_high_card(); }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::Card;
    use crate::suit::Suit;
    use crate::hole_cards::HoleCards;

    #[test]
    fn test_flush_is_greater_than_straight() {
        assert!(HandRank::Flush(CardArray::new()) > HandRank::Straight(CardArray::new()));
    }

    #[test]
    fn test_trips_is_smaller_than_quad() {
        assert!(HandRank::ThreeOfAKind(CardArray::new()) < HandRank::FourOfAKind(CardArray::new()));
    }

    #[test]
    fn test_compare_straight_flush() {
        let player = HoleCards::new(&Card::new(2, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(8, Suit::Clubs));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Clubs),
            Card::new(5, Suit::Clubs),
            Card::new(6, Suit::Clubs),
            Card::new(14, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_straight_flush(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_straight_flush(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_straight_flush() {
        let player = HoleCards::new(&Card::new(12, Suit::Clubs), &Card::new(14, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(13, Suit::Clubs), &Card::new(2, Suit::Clubs));
        let board = Board::from_vec(vec![
            Card::new(2, Suit::Clubs),
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Clubs),
            Card::new(5, Suit::Clubs),
            Card::new(6, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        assert_eq!(player_rank.is_straight_flush(), true);
        assert_eq!(opponent_rank.is_straight_flush(), true);
        assert_eq!(player_wins(player, opponent, board), None);
    }

    #[test]
    fn test_compare_quad_board() {
        let player = HoleCards::new(&Card::new(13, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(8, Suit::Clubs));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(3, Suit::Spades),
            Card::new(2, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_four_of_a_kind(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_four_of_a_kind(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(false));
    }

    #[test]
    fn test_equal_quad_board() {
        let player = HoleCards::new(&Card::new(13, Suit::Clubs), &Card::new(8, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(13, Suit::Diamonds), &Card::new(8, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(3, Suit::Spades),
            Card::new(2, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_four_of_a_kind(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_four_of_a_kind(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), None);
    }

    #[test]
    fn test_compare_full_house() {
        let player = HoleCards::new(&Card::new(8, Suit::Clubs), &Card::new(2, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(8, Suit::Spades),
            Card::new(8, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_full_house(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_full_house(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_full_house() {
        let player = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(12, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(8, Suit::Spades),
            Card::new(8, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_full_house(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_full_house(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), None);
    }

    #[test]
    fn test_compare_flush() {
        let player = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(12, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Clubs),
            Card::new(5, Suit::Clubs),
            Card::new(8, Suit::Clubs),
            Card::new(9, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_flush(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_flush(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_flush() {
        let player = HoleCards::new(&Card::new(14, Suit::Diamonds), &Card::new(13, Suit::Diamonds));
        let opponent = HoleCards::new(&Card::new(12, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Clubs),
            Card::new(5, Suit::Clubs),
            Card::new(8, Suit::Clubs),
            Card::new(9, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_flush(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_flush(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), None);
    }

    #[test]
    fn test_compare_straight() {
        let player = HoleCards::new(&Card::new(2, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(8, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Diamonds),
            Card::new(5, Suit::Diamonds),
            Card::new(6, Suit::Spades),
            Card::new(7, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_straight(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_straight(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_straight() {
        let player = HoleCards::new(&Card::new(2, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(9, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(4, Suit::Spades),
            Card::new(5, Suit::Spades),
            Card::new(6, Suit::Spades),
            Card::new(7, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_straight(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_straight(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_compare_trips() {
        let player = HoleCards::new(&Card::new(2, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(8, Suit::Diamonds), &Card::new(11, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(6, Suit::Spades),
            Card::new(7, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_three_of_a_kind(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_three_of_a_kind(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_trips() {
        let player = HoleCards::new(&Card::new(5, Suit::Clubs), &Card::new(4, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(4, Suit::Diamonds), &Card::new(2, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(3, Suit::Hearts),
            Card::new(7, Suit::Clubs),
            Card::new(13, Suit::Spades),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_three_of_a_kind(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_three_of_a_kind(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_compare_two_pair() {
        let player = HoleCards::new(&Card::new(13, Suit::Clubs), &Card::new(13, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(6, Suit::Diamonds), &Card::new(14, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(2, Suit::Hearts),
            Card::new(2, Suit::Spades),
            Card::new(5, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_two_pair(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_two_pair(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_two_pair() {
        let player = HoleCards::new(&Card::new(6, Suit::Clubs), &Card::new(7, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(6, Suit::Diamonds), &Card::new(7, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(3, Suit::Clubs),
            Card::new(3, Suit::Diamonds),
            Card::new(2, Suit::Hearts),
            Card::new(2, Suit::Spades),
            Card::new(5, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_two_pair(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_two_pair(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), None);
    }

    #[test]
    fn test_compare_one_pair() {
        let player = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(6, Suit::Spades));
        let board = Board::from_vec(vec![
            Card::new(2, Suit::Diamonds),
            Card::new(6, Suit::Hearts),
            Card::new(5, Suit::Spades),
            Card::new(7, Suit::Clubs),
            Card::new(8, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_one_pair(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_one_pair(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_equal_one_pair() {
        let player = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(7, Suit::Clubs), &Card::new(4, Suit::Spades));
        let board = Board::from_vec(vec![
            Card::new(14, Suit::Diamonds),
            Card::new(13, Suit::Hearts),
            Card::new(12, Suit::Clubs),
            Card::new(7, Suit::Clubs),
            Card::new(8, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_one_pair(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_one_pair(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(true));
    }

    #[test]
    fn test_compare_high_card() {
        let player = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(4, Suit::Clubs), &Card::new(2, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(6, Suit::Spades),
            Card::new(5, Suit::Hearts),
            Card::new(8, Suit::Spades),
            Card::new(9, Suit::Clubs),
            Card::new(10, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_high_card(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_high_card(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), Some(false));
    }

    #[test]
    fn test_equal_high_card() {
        let player = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(3, Suit::Clubs));
        let opponent = HoleCards::new(&Card::new(14, Suit::Clubs), &Card::new(2, Suit::Diamonds));
        let board = Board::from_vec(vec![
            Card::new(6, Suit::Spades),
            Card::new(5, Suit::Hearts),
            Card::new(8, Suit::Spades),
            Card::new(9, Suit::Clubs),
            Card::new(10, Suit::Clubs),
        ]);

        let player_rank = calculate_hand_rank(&player, &board);
        let opponent_rank = calculate_hand_rank(&opponent, &board);

        debug_assert_eq!(player_rank.is_high_card(), true, "Player rank: {:}", player_rank);
        debug_assert_eq!(opponent_rank.is_high_card(), true, "Opponent rank: {:}", opponent_rank);
        assert_eq!(player_wins(player, opponent, board), None);
    }
}