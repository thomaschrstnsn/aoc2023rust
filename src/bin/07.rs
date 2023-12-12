use std::{collections::HashSet, num::ParseIntError};

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, Hash)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    One,
}

fn discriminator(card: &Card) -> u8 {
    match card {
        Card::Ace => 14,
        Card::King => 13,
        Card::Queen => 12,
        Card::Jack => 11,
        Card::Ten => 10,
        Card::Nine => 9,
        Card::Eight => 8,
        Card::Seven => 7,
        Card::Six => 6,
        Card::Five => 5,
        Card::Four => 4,
        Card::Three => 3,
        Card::Two => 2,
        Card::One => 1,
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand(HashSet<Card>);

type Bid = u32;

#[derive(Debug, PartialEq)]
struct Input(Hand, Bid);

fn parse_card(c: char) -> Option<Card> {
    match c {
        'A' => Some(Card::Ace),
        'K' => Some(Card::King),
        'Q' => Some(Card::Queen),
        'J' => Some(Card::Jack),
        'T' => Some(Card::Ten),
        '9' => Some(Card::Nine),
        '8' => Some(Card::Eight),
        '7' => Some(Card::Seven),
        '6' => Some(Card::Six),
        '5' => Some(Card::Five),
        '4' => Some(Card::Four),
        '3' => Some(Card::Three),
        '2' => Some(Card::Two),
        '1' => Some(Card::One),
        _ => None,
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidCard(char),
    InvalidHandLength,
    InvalidBid(ParseIntError),
    InvalidStruct,
}

fn parse(lines: &str) -> Result<Vec<Input>, ParseError> {
    lines.lines().map(|l| l.parse::<Input>()).collect()
}

impl std::str::FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').ok_or(ParseError::InvalidStruct)?;

        let hand = hand_str
            .chars()
            .map(|c| parse_card(c).ok_or(ParseError::InvalidCard(c)))
            .collect::<Result<HashSet<_>, Self::Err>>()?;

        let bid = bid_str
            .parse::<u32>()
            .map_err(|pie| ParseError::InvalidBid(pie))?;

        if hand_str.len() != 5 {
            return Err(ParseError::InvalidHandLength);
        }

        Ok(Input(Hand(hand), bid))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("should parse");

        let expected = Input(
            Hand(HashSet::<Card>::from([Three, Two, Ten, Three, King])),
            765,
        );
        assert_eq!(result[0], expected);
    }
}
