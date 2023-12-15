use itertools::Itertools;
use std::{
    cmp::{Ord, Ordering},
    hash::Hash,
    num::ParseIntError,
};
use std::collections::{HashMap};

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
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Ord)]
enum JokerCard {
    Ace = 14,
    King = 13,
    Queen = 12,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    JokerJack = 1,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T>(Vec<T>);

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

trait Rankable<T> {
    fn rank(&self) -> HandRanking;
    fn cards(&self) -> &Vec<T>;
}

impl Rankable<Card> for Hand<Card> {
    fn rank(&self) -> HandRanking {
        use HandRanking::*;
        let Hand(hand) = self;
        let counts: HashMap<_, usize> = hand.iter().counts();

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

    fn cards(&self) -> &Vec<Card> {
        let Hand(cards) = self;
        cards
    }
}

impl Rankable<JokerCard> for Hand<JokerCard> {
    fn rank(&self) -> HandRanking {
        use HandRanking::*;
        let Hand(hand) = self;
        let mut counts: HashMap<_, usize> = hand.iter().counts();

        let jacks = *counts.get(&JokerCard::JokerJack).unwrap_or(&0usize);
        counts.remove_entry(&JokerCard::JokerJack);

        let mut freq: Vec<&usize> = counts.values().collect();
        freq.sort();

        let mut freq_iter = freq.into_iter().rev();
        match (freq_iter.next(), freq_iter.next(), jacks) {
            (Some(x), _, j) if x + j == 5 => FiveOfAKind,
            (None, _, 5) => FiveOfAKind,
            (Some(x), _, j) if x + j == 4 => FourOfAKind,
            (None, _, 4) => FourOfAKind,
            (Some(3), Some(2), 0) => FullHouse,
            (Some(2), Some(2), 1) => FullHouse,
            (Some(3), Some(1), 1) => FullHouse,
            (Some(2), Some(1), 2) => FullHouse,
            (Some(1), Some(1), 3) => FullHouse,
            (Some(x), _, j) if x + j == 3 => ThreeOfAKind,
            (Some(2), Some(2), 0) => TwoPair,
            (Some(2), Some(1), 1) => TwoPair,
            (Some(1), Some(1), 2) => TwoPair,
            (Some(x), _, j) if x + j == 2 => OnePair,
            _ => HighCard,
        }
    }

    fn cards(&self) -> &Vec<JokerCard> {
        let Hand(cards) = self;
        cards
    }
}

fn order_hands<HandT: Rankable<T>, T: std::fmt::Debug + Eq + Hash + Ord>(
    this: &HandT,
    other: &HandT,
) -> Ordering {
    let self_rank = this.rank();
    let other_rank = other.rank();
    match self_rank.cmp(&other_rank) {
        Ordering::Equal => {
            let self_cards = this.cards();
            let other_cards = other.cards();
            let first_diff = std::iter::zip(self_cards, other_cards)
                .map(|(a, b)| a.cmp(b))
                .find(|&o| o != Ordering::Equal);
            first_diff.unwrap_or(Ordering::Equal)
        }
        less_or_greater => less_or_greater,
    }
}

type Bid = u32;

#[derive(Debug, PartialEq)]
struct Input(Hand<Card>, Bid);

#[derive(Debug, PartialEq)]
struct JokerInput(Hand<JokerCard>, Bid);

fn convert_to_joker_card(c: &Card) -> JokerCard {
    match c {
        Card::Ace => JokerCard::Ace,
        Card::King => JokerCard::King,
        Card::Queen => JokerCard::Queen,
        Card::Jack => JokerCard::JokerJack,
        Card::Ten => JokerCard::Ten,
        Card::Nine => JokerCard::Nine,
        Card::Eight => JokerCard::Eight,
        Card::Seven => JokerCard::Seven,
        Card::Six => JokerCard::Six,
        Card::Five => JokerCard::Five,
        Card::Four => JokerCard::Four,
        Card::Three => JokerCard::Three,
        Card::Two => JokerCard::Two,
    }
}

fn convert_to_joker(input: &Input) -> JokerInput {
    let Input(Hand(cards), bid) = input;

    let joker_cards: Vec<_> = cards.iter().map(convert_to_joker_card).collect();
    JokerInput(Hand(joker_cards), *bid)
}

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
        _ => None,
    }
}

#[derive(Debug)]
enum ParseError {
    InvalidCard(char),
    HandLengthInvalid(usize),
    BidInvalid(ParseIntError),
    StructuralProblem,
}

fn parse(lines: &str) -> Result<Vec<Input>, ParseError> {
    lines.lines().map(|l| l.parse::<Input>()).collect()
}

impl std::str::FromStr for Input {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (hand_str, bid_str) = s.split_once(' ').ok_or(ParseError::StructuralProblem)?;

        let hand = hand_str
            .chars()
            .map(|c| parse_card(c).ok_or(ParseError::InvalidCard(c)))
            .collect::<Result<Vec<_>, Self::Err>>()?;

        let bid = bid_str
            .parse::<u32>()
            .map_err(ParseError::BidInvalid)?;

        if hand_str.len() != 5 {
            return Err(ParseError::HandLengthInvalid(hand_str.len()));
        }

        Ok(Input(Hand(hand), bid))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut inputs = parse(input).expect("parsable input");

    inputs.sort_by(|Input(a_hand, _), Input(b_hand, _)| {
        order_hands(a_hand, b_hand)
    });

    Some(
        inputs
            .into_iter()
            .enumerate()
            .map(|(index, Input(_, bid))| (index as u32 + 1) * bid)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let normal_inputs = parse(input).expect("parsable input");
    let mut inputs: Vec<_> = normal_inputs.iter().map(convert_to_joker).collect();

    inputs.sort_by(|JokerInput(a_hand, _), JokerInput(b_hand, _)| {
        order_hands(a_hand, b_hand)
    });

    Some(
        inputs
            .into_iter()
            .enumerate()
            .map(|(index, JokerInput(_, bid))| (index as u32 + 1) * bid)
            .sum(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
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
        use Card::*;
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("should parse");

        let expected = Input(Hand(vec![Three, Two, Ten, Three, King]), 765);
        assert_eq!(result[0], expected);
    }

    #[test]
    fn test_joker_rank() {
        use JokerCard::*;
        assert_eq!(
            Hand(vec!(JokerJack, Ace, Ace, Ace, Ace)).rank(),
            FiveOfAKind
        );
        assert_eq!(
            Hand(vec!(Two, Ace, JokerJack, Ace, Ace)).rank(),
            FourOfAKind
        );
        assert_eq!(Hand(vec!(Two, Ace, JokerJack, Two, Ace)).rank(), FullHouse);
        assert_eq!(
            Hand(vec!(Two, JokerJack, Ace, Three, Ace)).rank(),
            ThreeOfAKind
        );
        assert_eq!(Hand(vec!(Two, Ace, Two, Three, Ace)).rank(), TwoPair);
        assert_eq!(Hand(vec!(Six, JokerJack, Two, Three, Ace)).rank(), OnePair);
        assert_eq!(Hand(vec!(Six, Five, Two, Three, Ace)).rank(), HighCard);
    }

    #[test]
    fn test_rank() {
        use Card::*;
        assert_eq!(Hand(vec!(Ace, Ace, Ace, Ace, Ace)).rank(), FiveOfAKind);
        assert_eq!(Hand(vec!(Two, Ace, Ace, Ace, Ace)).rank(), FourOfAKind);
        assert_eq!(Hand(vec!(Two, Ace, Ace, Two, Ace)).rank(), FullHouse);
        assert_eq!(Hand(vec!(Two, Ace, Ace, Three, Ace)).rank(), ThreeOfAKind);
        assert_eq!(Hand(vec!(Two, Ace, Two, Three, Ace)).rank(), TwoPair);
        assert_eq!(Hand(vec!(Six, Ace, Two, Three, Ace)).rank(), OnePair);
        assert_eq!(Hand(vec!(Six, Five, Two, Three, Ace)).rank(), HighCard);
    }

    #[test]
    fn test_cmp_hand() {
        use Card::*;
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
                &Hand(vec![Two, Ace, Ace, Ace, Ace]),
            ),
            Ordering::Greater
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Two, Ace, Ace, Ace, Two]),
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
            ),
            Ordering::Less
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
                &Hand(vec![Two, Ace, Two, Ace, Ace]),
            ),
            Ordering::Greater
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ten, King, Queen, Two]),
                &Hand(vec![Ace, Ten, King, Queen, Two]),
            ),
            Ordering::Equal
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Two, Ace, Ace, Ace, Ace]),
                &Hand(vec![Three, Two, Two, Two, Two]),
            ),
            Ordering::Less
        );
    }
}
