use itertools::Itertools;
use std::{
    cmp::{self, Ord, Ordering},
    collections::HashMap,
    hash::Hash,
    num::ParseIntError,
};

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
enum Card {
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
    One = 1,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T: Hash>(Vec<T>);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandRanking {
    FiveOfAKind = 10,
    FourOfAKind = 9,
    FullHouse = 8,
    ThreeOfAKind = 7,
    TwoPair = 6,
    OnePair = 5,
    HighCard = 4,
}

fn rank<T: Eq + Hash>(Hand(hand): &Hand<T>) -> HandRanking {
    use HandRanking::*;
    let counts: HashMap<&T, usize> = hand.into_iter().counts();

    let mut freq: Vec<usize> = counts.into_values().collect();
    freq.sort();

    let mut freq_iter = freq.into_iter().rev();
    match (freq_iter.next(), freq_iter.next()) {
        (Some(5), _) => FiveOfAKind,
        (Some(4), _) => FourOfAKind,
        (Some(3), Some(2)) => FullHouse,
        (Some(3), _) => ThreeOfAKind,
        (Some(2), Some(2)) => TwoPair,
        (Some(2), _) => OnePair,
        _ => HighCard,
    }
}

impl<T: PartialEq + Ord + Hash> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: cmp::Eq + Ord + Hash> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_rank = rank(self);
        let other_rank = rank(other);
        match self_rank.cmp(&other_rank) {
            Ordering::Equal => {
                let Hand(self_cards) = self;
                let Hand(other_cards) = other;
                let first_diff = std::iter::zip(self_cards, other_cards)
                    .map(|(a, b)| a.cmp(&b))
                    .skip_while(|&o| o == Ordering::Equal)
                    .next();
                first_diff.unwrap_or(Ordering::Equal)
            }
            less_or_greater => less_or_greater,
        }
    }
}

type Bid = u32;

#[derive(Debug, PartialEq)]
struct Input(Hand<Card>, Bid);

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
            .collect::<Result<Vec<_>, Self::Err>>()?;

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
    let mut inputs = parse(input).expect("parsable input");

    inputs.sort_by(|Input(a_hand, _), Input(b_hand, _)| a_hand.cmp(&b_hand));

    Some(
        inputs
            .into_iter()
            .enumerate()
            .map(|(index, Input(_, bid))| (index as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use Card::*;
    use HandRanking::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }

    #[test]
    fn test_parse() {
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("should parse");

        let expected = Input(Hand(vec![Three, Two, Ten, Three, King]), 765);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_rank() {
        assert_eq!(rank(&Hand(vec!(Ace, Ace, Ace, Ace, Ace))), FiveOfAKind);
        assert_eq!(rank(&Hand(vec!(One, Ace, Ace, Ace, Ace))), FourOfAKind);
        assert_eq!(rank(&Hand(vec!(One, Ace, Ace, One, Ace))), FullHouse);
        assert_eq!(rank(&Hand(vec!(One, Ace, Ace, Three, Ace))), ThreeOfAKind);
        assert_eq!(rank(&Hand(vec!(One, Ace, One, Three, Ace))), TwoPair);
        assert_eq!(rank(&Hand(vec!(One, Ace, Two, Three, Ace))), OnePair);
        assert_eq!(rank(&Hand(vec!(One, Five, Two, Three, Ace))), HighCard);
    }

    #[test]
    fn test_cmp_hand() {
        assert!(Hand(vec![Ace, Ace, Ace, Ace, One]) > Hand(vec![One, Ace, Ace, Ace, Ace]));
        assert!(Hand(vec![One, Ace, Ace, Ace, One]) < Hand(vec![Ace, Ace, Ace, Ace, One]));
        assert!(Hand(vec![Ace, Ace, Ace, Ace, One]) > Hand(vec![One, Ace, Two, Ace, Ace]));
        assert!(Hand(vec![Ace, Ten, King, Queen, One]) == Hand(vec![Ace, Ten, King, Queen, One]));
        assert!(Hand(vec![One, Ace, Ace, Ace, Ace]) < Hand(vec![Three, One, One, One, One]));
    }
}
