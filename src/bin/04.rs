use std::{
    collections::{HashMap, HashSet},
    num::ParseIntError,
};

advent_of_code::solution!(4);

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
    number: u16,
    winners: HashSet<u8>,
    drawn: HashSet<u8>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    InvalidStructure,
    NotInt(ParseIntError),
    NotInt2,
}

impl std::str::FromStr for Card {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rest = s.strip_prefix("Card").ok_or(ParseError::InvalidStructure)?;
        let (card_num_str, rest) = rest.split_once(':').ok_or(ParseError::InvalidStructure)?;

        let card_num = card_num_str
            .trim()
            .parse::<u16>()
            .map_err(ParseError::NotInt)?;

        let (winners_strs, drawn_strs) =
            rest.split_once('|').ok_or(ParseError::InvalidStructure)?;

        let winners = u8s(winners_strs)
            .map_err(ParseError::NotInt)?
            .into_iter()
            .collect();
        let drawn = u8s(drawn_strs)
            .map_err(ParseError::NotInt)?
            .into_iter()
            .collect();

        Ok(Card {
            number: card_num,
            winners,
            drawn,
        })
    }
}

fn u8s(s: &str) -> Result<Vec<u8>, ParseIntError> {
    s.trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u8>())
        .collect()
}

fn parse_cards(s: &str) -> Result<Vec<Card>, ParseError> {
    s.lines().map(|line| line.parse::<Card>()).collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse_cards(input).expect("parseeee");

    Some(
        cards
            .into_iter()
            .map(|card| {
                let count = card.matches();
                if count > 0 {
                    1 << (count - 1)
                } else {
                    0
                }
            })
            .sum(),
    )
}

impl Card {
    fn matches(&self) -> u32 {
        let count = self.winners.intersection(&self.drawn).count();

        u32::try_from(count).unwrap()
    }
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse_cards(input).expect("parseeee");

    let min = cards
        .iter()
        .map(|c| c.number)
        .min()
        .expect("min must exist");
    let max = cards
        .iter()
        .map(|c| c.number)
        .max()
        .expect("max must exist");

    let card_map: HashMap<u16, &Card> = cards.iter().map(|c| (c.number, c)).collect();
    let mut card_count: HashMap<u16, u32> = cards.iter().map(|c| (c.number, 1)).collect();

    for i in min..max {
        let card = card_map[&i];
        let mult = card_count[&i];

        let matches = card.matches();
        for add_index in 0..(matches as u16) {
            if let Some(count_to_update) = card_count.get_mut(&(i + add_index + 1)) {
                *count_to_update += mult;
            } else {
                dbg!("could not find", i);
            }
        }
    }

    Some(card_count.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(30));
    }

    #[test]
    fn test_parse() {
        let inp = advent_of_code::template::read_file("examples", DAY);
        let input = inp.lines().next().expect("one line");

        let actual = input.parse::<Card>().expect("should parse");

        let expected = Card {
            number: 1,
            winners: vec![41, 48, 83, 86, 17].into_iter().collect(),
            drawn: vec![83, 86, 6, 31, 17, 9, 48, 53].into_iter().collect(),
        };

        assert_eq!(actual, expected);
    }
}
