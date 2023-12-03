advent_of_code::solution!(2);

#[derive(Debug, Eq, PartialEq)]
pub struct Draw {
    red: u32,
    blue: u32,
    green: u32,
}

#[derive(Debug, Eq, PartialEq)]
pub struct Game {
    id: u32,
    draws: Vec<Draw>,
}

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NotInt,
    InvalidColor,
    CannotSplit(char),
    NotAGame
}

fn parse_games(s: &str) -> Result<Vec<Game>, ParseError> {
    let games : Result<Vec<Game>, ParseError> = s.split('\n').filter(|s| !s.is_empty()).map(|line| line.parse::<Game>()).collect();
    games
}

impl FromStr for Game {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, draws) = s.split_once(':').ok_or(ParseError::CannotSplit(':'))?;
        let (kw, id_str) = game.split_once(' ').ok_or(ParseError::CannotSplit(' '))?;
        let id = id_str.parse::<u32>().map_err(|_| ParseError::NotInt)?;
        if kw == "Game" {
            let draws_result : Result<Vec<Draw>, Self::Err> = draws.split(';').map(|d| d.parse::<Draw>()).collect();
            let draws = draws_result?;
            Ok(Game {id, draws})
        } else {
            Err(ParseError::NotAGame)
        }
    }
}

impl FromStr for Draw {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut draw = Draw {red: 0, green: 0, blue: 0};
        for c in s.trim().split(',') {
            let (value_str, color_str) = c.trim().split_once(' ').ok_or(ParseError::CannotSplit(' '))?;
            let value = value_str.trim().parse::<u32>().map_err(|_| ParseError::NotInt)?;
            match color_str.trim() {
                "blue" => {draw.blue = value},
                "red" => {draw.red = value},
                "green" => {draw.green = value},
                _ => todo!(),
            }
        }
        Ok(draw)
    }
}

fn valid_game(bag: &Draw, candidate: &Game) -> bool {
    candidate.draws.iter().all(|d| valid_draw(bag, d))
}

fn valid_draw(bag: &Draw, candidate: &Draw) -> bool {
    candidate.red <= bag.red && candidate.blue <= bag.blue && candidate.green <= bag.green
}

pub fn part_one(input: &str) -> Option<u32> {
    let games = parse_games(input).ok()?;
    let bag = Draw {red: 12, green: 13, blue: 14};

    let possible_games = games.iter().filter(|g| valid_game(&bag, g));
    Some(possible_games.map(|g| g.id).sum())
}

fn power(draw: &Draw) -> u32 {
    draw.red * draw.green * draw.blue
}

fn minimal_bag(game: &Game) -> Draw {
    let red = game.draws.iter().max_by_key(|d| d.red).map(|d| d.red).unwrap();
    let green = game.draws.iter().max_by_key(|d| d.green).map(|d| d.green).unwrap();
    let blue = game.draws.iter().max_by_key(|d| d.blue).map(|d| d.blue).unwrap();

    Draw {red, green, blue}
}

pub fn part_two(input: &str) -> Option<u32> {
    let games = parse_games(input).ok()?;
    let minimal_bags = games.iter().map(minimal_bag);
    Some(minimal_bags.map(|d| power(&d)).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }

    #[test]
    fn parse_draw() {
        let result = "1 green, 2 red, 3 blue".parse::<Draw>();
        assert_eq!(result, Ok(Draw{red: 2, green: 1, blue: 3}));
    }
    #[test]
    fn parse_game() {
        let input = "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red";
        let result = input.parse::<Game>();
        assert_eq!(result, Ok(
                Game{id: 4,
                    draws: vec!(
                        Draw{red: 3, green: 1, blue: 6},
                        Draw{green: 3, red: 6, blue: 0},
                        Draw{ green:3, blue: 15, red: 14})}));
    }
}
