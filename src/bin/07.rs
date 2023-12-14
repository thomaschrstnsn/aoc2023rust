use itertools::Itertools;
use std::{
    cmp::{self, Ord, Ordering},
    collections::HashMap,
    hash::Hash,
    num::ParseIntError,
};

advent_of_code::solution!(7);

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Ord, Clone)]
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

#[derive(Debug, Eq, PartialEq, PartialOrd, Hash, Ord, Clone)]
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

#[derive(Hash, Debug)]
struct JokerIsWild(JokerCard);

impl PartialEq for JokerIsWild {
    fn eq(&self, other: &Self) -> bool {
        let JokerIsWild(self_card) = self;
        let JokerIsWild(other_card) = other;
        match (self_card, other_card) {
            (JokerCard::JokerJack, _) => true,
            (_, JokerCard::JokerJack) => true,
            (s, o) => s == o,
        }
    }
}

impl Eq for JokerIsWild {}

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

fn regular_eqt(c: &Card) -> Box<Card> {
    Box::new(c.clone())
}

fn joker_eqt(c: &JokerCard) -> Box<JokerIsWild> {
    Box::new(JokerIsWild(c.clone()))
}

fn rank<T, EqT: Eq + Hash + std::fmt::Debug>(Hand(hand): &Hand<T>, eq_comparer: fn(&T) -> Box<EqT>) -> HandRanking {
    use HandRanking::*;
    let counts: HashMap<_, usize> = hand.into_iter().map(eq_comparer).counts();
    dbg!(&counts);

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

fn regular_tie_breaker(a: &Card, b: &Card) -> Ordering {
    a.cmp(&b)
}

fn joker_tie_breaker(a: &JokerCard, b: &JokerCard) -> Ordering {
    a.cmp(&b)
}

fn order_hands<T, EqT: Eq + Hash + std::fmt::Debug>(
    this: &Hand<T>,
    other: &Hand<T>,
    tie_breaker: fn(&T, &T) -> Ordering,
    card_eqt: fn(&T) -> Box<EqT>,
) -> Ordering {
    let self_rank = rank(this, card_eqt);
    let other_rank = rank(other, card_eqt);
    match self_rank.cmp(&other_rank) {
        Ordering::Equal => {
            let Hand(self_cards) = this;
            let Hand(other_cards) = other;
            let first_diff = std::iter::zip(self_cards, other_cards)
                .map(|(a, b)| tie_breaker(a, b))
                .skip_while(|&o| o == Ordering::Equal)
                .next();
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

    let joker_cards : Vec<_> = cards.into_iter().map(convert_to_joker_card).collect();
    JokerInput(Hand(joker_cards), bid.clone())
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

    inputs.sort_by(|Input(a_hand, _), Input(b_hand, _)| {
        order_hands(&a_hand, &b_hand, regular_tie_breaker, regular_eqt)
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
    let mut inputs : Vec<_> = normal_inputs.iter().map(convert_to_joker).collect();

    inputs.sort_by(|JokerInput(a_hand, _), JokerInput(b_hand, _)| {
        order_hands::<JokerCard, JokerIsWild>(&a_hand, &b_hand, joker_tie_breaker, joker_eqt)
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
            rank(&Hand(vec!(JokerJack, Ace, Ace, Ace, Ace)), joker_eqt),
            FiveOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, JokerJack, Ace, Ace)), joker_eqt),
            FourOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, JokerJack, Two, Ace)), joker_eqt),
            FullHouse
        );
        assert_eq!(
            rank(&Hand(vec!(Two, JokerJack, Ace, Three, Ace)), joker_eqt),
            ThreeOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, Two, Three, Ace)), joker_eqt),
            TwoPair
        );
        assert_eq!(
            rank(&Hand(vec!(Six, JokerJack, Two, Three, Ace)), joker_eqt),
            OnePair
        );
        assert_eq!(
            rank(&Hand(vec!(Six, Five, Two, Three, Ace)), joker_eqt),
            HighCard
        );
    }

    #[test]
    fn test_rank() {
        use Card::*;
        assert_eq!(
            rank(&Hand(vec!(Ace, Ace, Ace, Ace, Ace)), regular_eqt),
            FiveOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, Ace, Ace, Ace)), regular_eqt),
            FourOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, Ace, Two, Ace)), regular_eqt),
            FullHouse
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, Ace, Three, Ace)), regular_eqt),
            ThreeOfAKind
        );
        assert_eq!(
            rank(&Hand(vec!(Two, Ace, Two, Three, Ace)), regular_eqt),
            TwoPair
        );
        assert_eq!(
            rank(&Hand(vec!(Six, Ace, Two, Three, Ace)), regular_eqt),
            OnePair
        );
        assert_eq!(
            rank(&Hand(vec!(Six, Five, Two, Three, Ace)), regular_eqt),
            HighCard
        );
    }

    #[test]
    fn test_cmp_hand() {
        use Card::*;
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
                &Hand(vec![Two, Ace, Ace, Ace, Ace]),
                regular_tie_breaker,
                regular_eqt
            ),
            Ordering::Greater
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Two, Ace, Ace, Ace, Two]),
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
                regular_tie_breaker,
                regular_eqt
            ),
            Ordering::Less
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ace, Ace, Ace, Two]),
                &Hand(vec![Two, Ace, Two, Ace, Ace]),
                regular_tie_breaker,
                regular_eqt
            ),
            Ordering::Greater
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Ace, Ten, King, Queen, Two]),
                &Hand(vec![Ace, Ten, King, Queen, Two]),
                regular_tie_breaker,
                regular_eqt
            ),
            Ordering::Equal
        );
        assert_eq!(
            order_hands(
                &Hand(vec![Two, Ace, Ace, Ace, Ace]),
                &Hand(vec![Three, Two, Two, Two, Two]),
                regular_tie_breaker,
                regular_eqt
            ),
            Ordering::Less
        );
    }
}
