use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs::OpenOptions;

pub mod cli;
pub mod utils;
// const POSITIONS: [&str; 8] = ["EP1", "EP2", "EP3", "LJ", "HJ", "CO", "BTN", "SB"];
pub enum Position {
    EarlyPosition1,
    EarlyPosition2,
    EarlyPosition3,
    LowJack,
    HighJack,
    Cutoff,
    Button,
    SmallBlind,
}

#[derive(Deserialize, Debug)]
pub struct Schema {
    raise: String,
    raise_or_fold: String,
}

#[derive(Deserialize, Debug)]
pub struct PositionInput {
    position: String,
    schema: Schema,
}

pub fn load_range_file<'a>(filename: &str) -> Result<Vec<PositionInput>, Box<dyn Error>> {
    let range_file = OpenOptions::new().read(true).open(filename)?;

    let position_inputs: Vec<PositionInput> = serde_json::from_reader(range_file)?;
    Ok(position_inputs)
}

pub mod parser {
    use super::*;
    use std::{char, fmt::Display};

    const VALID_HANDS: [Hand; 169] = [
        Hand(CardType::Ace, CardType::Ace, SuitedType::None),
        Hand(CardType::Ace, CardType::King, SuitedType::Suited),
        Hand(CardType::Ace, CardType::King, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Queen, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Queen, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Jack, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Jack, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Ten, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Ten, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Nine, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Nine, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Eight, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Six, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Five, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Four, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Three, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Ace, CardType::Two, SuitedType::Suited),
        Hand(CardType::Ace, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::King, CardType::King, SuitedType::None),
        Hand(CardType::King, CardType::Queen, SuitedType::Suited),
        Hand(CardType::King, CardType::Queen, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Jack, SuitedType::Suited),
        Hand(CardType::King, CardType::Jack, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Ten, SuitedType::Suited),
        Hand(CardType::King, CardType::Ten, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Nine, SuitedType::Suited),
        Hand(CardType::King, CardType::Nine, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Eight, SuitedType::Suited),
        Hand(CardType::King, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Seven, SuitedType::Suited),
        Hand(CardType::King, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Six, SuitedType::Suited),
        Hand(CardType::King, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Five, SuitedType::Suited),
        Hand(CardType::King, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Four, SuitedType::Suited),
        Hand(CardType::King, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Three, SuitedType::Suited),
        Hand(CardType::King, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::King, CardType::Two, SuitedType::Suited),
        Hand(CardType::King, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Queen, SuitedType::None),
        Hand(CardType::Queen, CardType::Jack, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Jack, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Ten, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Ten, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Nine, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Nine, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Eight, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Six, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Five, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Four, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Three, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Queen, CardType::Two, SuitedType::Suited),
        Hand(CardType::Queen, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Jack, SuitedType::None),
        Hand(CardType::Jack, CardType::Ten, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Ten, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Nine, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Nine, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Eight, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Six, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Five, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Four, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Three, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Jack, CardType::Two, SuitedType::Suited),
        Hand(CardType::Jack, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Ten, SuitedType::None),
        Hand(CardType::Ten, CardType::Nine, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Nine, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Eight, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Six, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Five, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Four, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Three, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Ten, CardType::Two, SuitedType::Suited),
        Hand(CardType::Ten, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Nine, SuitedType::None),
        Hand(CardType::Nine, CardType::Eight, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Eight, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Six, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Five, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Four, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Three, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Nine, CardType::Two, SuitedType::Suited),
        Hand(CardType::Nine, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Eight, SuitedType::None),
        Hand(CardType::Eight, CardType::Seven, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Seven, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Six, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Five, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Four, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Three, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Eight, CardType::Two, SuitedType::Suited),
        Hand(CardType::Eight, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Seven, CardType::Seven, SuitedType::None),
        Hand(CardType::Seven, CardType::Six, SuitedType::Suited),
        Hand(CardType::Seven, CardType::Six, SuitedType::Offsuit),
        Hand(CardType::Seven, CardType::Five, SuitedType::Suited),
        Hand(CardType::Seven, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Seven, CardType::Four, SuitedType::Suited),
        Hand(CardType::Seven, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Seven, CardType::Three, SuitedType::Suited),
        Hand(CardType::Seven, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Seven, CardType::Two, SuitedType::Suited),
        Hand(CardType::Seven, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Six, CardType::Six, SuitedType::None),
        Hand(CardType::Six, CardType::Five, SuitedType::Suited),
        Hand(CardType::Six, CardType::Five, SuitedType::Offsuit),
        Hand(CardType::Six, CardType::Four, SuitedType::Suited),
        Hand(CardType::Six, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Six, CardType::Three, SuitedType::Suited),
        Hand(CardType::Six, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Six, CardType::Two, SuitedType::Suited),
        Hand(CardType::Six, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Five, CardType::Five, SuitedType::None),
        Hand(CardType::Five, CardType::Four, SuitedType::Suited),
        Hand(CardType::Five, CardType::Four, SuitedType::Offsuit),
        Hand(CardType::Five, CardType::Three, SuitedType::Suited),
        Hand(CardType::Five, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Five, CardType::Two, SuitedType::Suited),
        Hand(CardType::Five, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Four, CardType::Four, SuitedType::None),
        Hand(CardType::Four, CardType::Three, SuitedType::Suited),
        Hand(CardType::Four, CardType::Three, SuitedType::Offsuit),
        Hand(CardType::Four, CardType::Two, SuitedType::Suited),
        Hand(CardType::Four, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Three, CardType::Three, SuitedType::None),
        Hand(CardType::Three, CardType::Two, SuitedType::Suited),
        Hand(CardType::Three, CardType::Two, SuitedType::Offsuit),
        Hand(CardType::Two, CardType::Two, SuitedType::None),
    ];

    #[derive(PartialEq, PartialOrd, Debug, Clone)]
    pub enum CardType {
        Ace = 14,
        King = 13,
        Queen = 12,
        Jack = 11,
        Ten = 10,
        Nine = 9,
        Eight = 8,
        Seven = 7,
        Six = 6,
        Five = 5,
        Four = 4,
        Three = 3,
        Two = 2,
    }

    impl Display for CardType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let card_display = match self {
                CardType::Ace => 'A',
                CardType::King => 'K',
                CardType::Queen => 'Q',
                CardType::Jack => 'J',
                CardType::Ten => 'T',
                CardType::Nine => '9',
                CardType::Eight => '8',
                CardType::Seven => '7',
                CardType::Six => '6',
                CardType::Five => '5',
                CardType::Four => '4',
                CardType::Three => '3',
                CardType::Two => '2',
            };
            write!(f, "{}", card_display)
        }
    }

    #[derive(PartialEq, Debug, Clone)]
    pub enum SuitedType {
        Suited,
        Offsuit,
        None,
    }

    impl Display for SuitedType {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let output = match self {
                SuitedType::Suited => "s",
                SuitedType::Offsuit => "o",
                SuitedType::None => "",
            };
            write!(f, "{}", output)
        }
    }

    #[derive(Debug, PartialEq, Clone)]
    pub struct Hand(CardType, CardType, SuitedType);

    impl Hand {
        pub fn new(first_card: CardType, second_card: CardType, suited_type: SuitedType) -> Self {
            if first_card == second_card {
                assert!(
                    suited_type == SuitedType::None,
                    "Pairs can't be {:?}, must be SuitedType::None",
                    suited_type
                );
            } else {
                assert!(suited_type != SuitedType::None, "Non-pairs can't be SuitedType::None, must be SuitedType::Suited or SuitedType::Offsuit")
            }
            Hand(first_card, second_card, suited_type)
        }

        pub fn has_card(&self, card: &CardType) -> bool {
            self.0 == *card || self.1 == *card
        }

        pub fn matches_suited_type(&self, suited: &SuitedType) -> bool {
            self.2 == *suited
        }

        pub fn is_pair(&self) -> bool {
            self.0 == self.1 && self.2 == SuitedType::None
        }

        pub fn suited_type(&self) -> &SuitedType {
            &self.2
        }

        pub fn from_hand(hand: &Hand, suited_type: SuitedType) -> Self {
            let cloned_hand = hand.to_owned();
            Hand(cloned_hand.0, cloned_hand.1, suited_type)
        }
    }

    impl Display for Hand {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}{}{}", self.0, self.1, self.2)
        }
    }

    impl PartialOrd for Hand {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            if self.is_pair() && other.is_pair() {
                // e.g. TT+ should return only pairs above or equal to TT (AA, KK, QQ, JJ, TT)
                return self.0.partial_cmp(&other.0);
            } else if self.matches_suited_type(&other.suited_type()) && self.0 == other.0 {
                // e.g. T2s+ should return only suited Tx hands (T9s, T8s, T7s, T6s, T5s, T4s, T3s, T2s)
                return self.1.partial_cmp(&other.1);
            } else if self.matches_suited_type(&SuitedType::None)
                && !self.is_pair()
                && self.0 == other.0
            {
                // e.g. 73+ should return all suited and offsuit hands greater than or equal to 73 (76s, 76o, 75s, 75o, 74s, 74o, 73s, 73o)
                return self.1.partial_cmp(&other.1);
            }
            None
        }
    }

    #[derive(Debug, PartialEq, PartialOrd)]
    enum ParseError {
        InvalidToken(String),
        EndOfLine,
    }

    impl Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ParseError::InvalidToken(string) => write!(f, "Unexpected token: {}", string),
                ParseError::EndOfLine => write!(f, "Unexpected end of line"),
            }
        }
    }

    type ParseResult<T> = Result<T, ParseError>;

    struct Parser {
        characters: Vec<char>,
        cursor: usize,
    }

    impl Parser {
        pub fn new(string: &str) -> Self {
            Self {
                characters: string.trim().chars().collect(),
                cursor: 0,
            }
        }

        pub fn peek(&self) -> Option<&char> {
            self.characters.get(self.cursor)
        }

        pub fn pop(&mut self) -> Option<&char> {
            match self.characters.get(self.cursor) {
                Some(character) => {
                    self.cursor += 1;
                    Some(character)
                }
                None => None,
            }
        }

        pub fn is_eof(&self) -> bool {
            self.characters.len() != 0 && self.characters.len() == self.cursor
        }

        pub fn next_matches(&mut self, target: char) -> bool {
            match self.peek() {
                Some(character) => {
                    if *character == target {
                        self.pop();
                        return true;
                    }
                    false
                }
                None => false,
            }
        }
    }

    impl Parser {
        pub fn parse_range(&mut self) -> ParseResult<Vec<Hand>> {
            let hand = self.parse_hand()?;
            match self.peek() {
                Some('+') => Ok(self.get_gte_hands(&hand)),
                Some('-') => Ok(self.get_lte_hands(&hand)),
                Some(other_character) => Err(ParseError::InvalidToken(other_character.to_string())),
                None => {
                    if !hand.is_pair() && hand.matches_suited_type(&SuitedType::None) {
                        Ok(vec![
                            Hand::from_hand(&hand, SuitedType::Suited),
                            Hand::from_hand(&hand, SuitedType::Offsuit),
                        ])
                    } else {
                        Ok(vec![hand])
                    }
                }
            }
        }

        pub fn parse_hand(&mut self) -> ParseResult<Hand> {
            let first_card = self.parse_card()?;
            let second_card = self.parse_card()?;
            let suited_type = if self.next_matches('s') {
                SuitedType::Suited
            } else if self.next_matches('o') {
                SuitedType::Offsuit
            } else {
                SuitedType::None
            };
            let result = Hand(first_card, second_card, suited_type);
            Ok(result)
        }

        pub fn parse_card(&mut self) -> ParseResult<CardType> {
            if self.next_matches('A') {
                Ok(CardType::Ace)
            } else if self.next_matches('K') {
                Ok(CardType::King)
            } else if self.next_matches('Q') {
                Ok(CardType::Queen)
            } else if self.next_matches('J') {
                Ok(CardType::Jack)
            } else if self.next_matches('T') {
                Ok(CardType::Ten)
            } else if self.next_matches('9') {
                Ok(CardType::Nine)
            } else if self.next_matches('8') {
                Ok(CardType::Eight)
            } else if self.next_matches('7') {
                Ok(CardType::Seven)
            } else if self.next_matches('6') {
                Ok(CardType::Six)
            } else if self.next_matches('5') {
                Ok(CardType::Five)
            } else if self.next_matches('4') {
                Ok(CardType::Four)
            } else if self.next_matches('3') {
                Ok(CardType::Three)
            } else if self.next_matches('2') {
                Ok(CardType::Two)
            } else {
                match self.peek() {
                    Some(character) => Err(ParseError::InvalidToken(character.to_string())),
                    None => Err(ParseError::EndOfLine),
                }
            }
        }

        fn get_gte_hands(&self, hand: &Hand) -> Vec<Hand> {
            if hand.matches_suited_type(&SuitedType::None) && !hand.is_pair() {
                let suited_hand = Hand::from_hand(hand, SuitedType::Suited);
                let offsuit_hand = Hand::from_hand(hand, SuitedType::Offsuit);
                return Vec::from(VALID_HANDS)
                    .into_iter()
                    .filter(|valid_hand| valid_hand >= &suited_hand || valid_hand >= &offsuit_hand)
                    .collect();
            }
            Vec::from(VALID_HANDS)
                .into_iter()
                .filter(|valid_hand| valid_hand >= hand)
                .collect()
        }

        fn get_lte_hands(&self, hand: &Hand) -> Vec<Hand> {
            if hand.matches_suited_type(&SuitedType::None) && !hand.is_pair() {
                let suited_hand = Hand::from_hand(hand, SuitedType::Suited);
                let offsuit_hand = Hand::from_hand(hand, SuitedType::Offsuit);
                return Vec::from(VALID_HANDS)
                    .into_iter()
                    .filter(|valid_hand| valid_hand <= &suited_hand || valid_hand <= &offsuit_hand)
                    .collect();
            }
            Vec::from(VALID_HANDS)
                .into_iter()
                .filter(|valid_hand| valid_hand <= hand && valid_hand.matches_suited_type(&hand.2))
                .collect()
        }
    }

    pub enum Error {
        Character(usize),
        EndOfLine,
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_partial_ord() {
            assert_eq!(
                Hand(CardType::Ace, CardType::Two, SuitedType::Suited)
                    < Hand(CardType::Ace, CardType::Three, SuitedType::Suited),
                true
            );
        }

        #[test]
        fn parses_nonpair_gte_range() {
            let mut parser = Parser::new("AQo+");
            let result = parser.parse_range();
            assert_eq!(
                Ok(vec![
                    Hand(CardType::Ace, CardType::King, SuitedType::Offsuit),
                    Hand(CardType::Ace, CardType::Queen, SuitedType::Offsuit),
                ]),
                result
            );
        }

        #[test]
        fn parses_nonpair_lte_range() {
            let mut parser = Parser::new("T9s-");
            let result = parser.parse_range();
            assert_eq!(
                Ok(vec![
                    Hand(CardType::Ten, CardType::Nine, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Eight, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Seven, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Six, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Five, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Four, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Three, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Two, SuitedType::Suited),
                ]),
                result
            );
        }

        #[test]
        fn parses_unsuited_hand() {
            let mut parser = Parser::new("T9");
            let result = parser.parse_range();
            assert_eq!(
                Ok(vec![
                    Hand(CardType::Ten, CardType::Nine, SuitedType::Suited),
                    Hand(CardType::Ten, CardType::Nine, SuitedType::Offsuit)
                ]),
                result
            )
        }

        #[test]
        fn parses_unsuited_range() {
            let mut parser = Parser::new("76-");
            let result = parser.parse_range();
            assert_eq!(
                result,
                Ok(vec![
                    Hand(CardType::Seven, CardType::Six, SuitedType::Suited),
                    Hand(CardType::Seven, CardType::Six, SuitedType::Offsuit),
                    Hand(CardType::Seven, CardType::Five, SuitedType::Suited),
                    Hand(CardType::Seven, CardType::Five, SuitedType::Offsuit),
                    Hand(CardType::Seven, CardType::Four, SuitedType::Suited),
                    Hand(CardType::Seven, CardType::Four, SuitedType::Offsuit),
                    Hand(CardType::Seven, CardType::Three, SuitedType::Suited),
                    Hand(CardType::Seven, CardType::Three, SuitedType::Offsuit),
                    Hand(CardType::Seven, CardType::Two, SuitedType::Suited),
                    Hand(CardType::Seven, CardType::Two, SuitedType::Offsuit),
                ])
            );
        }

        #[test]
        fn parses_pair_gte_range() {
            let mut parser = Parser::new("TT+");
            let result = parser.parse_range();
            assert_eq!(
                Ok(vec![
                    Hand(CardType::Ace, CardType::Ace, SuitedType::None),
                    Hand(CardType::King, CardType::King, SuitedType::None),
                    Hand(CardType::Queen, CardType::Queen, SuitedType::None),
                    Hand(CardType::Jack, CardType::Jack, SuitedType::None),
                    Hand(CardType::Ten, CardType::Ten, SuitedType::None),
                ]),
                result
            );
        }

        #[test]
        fn errs_on_invalid_token() {
            let mut parser = Parser::new("ATk");
            let result = parser.parse_range();
            assert_eq!(Err(ParseError::InvalidToken("k".to_string())), result);
        }
    }
}
