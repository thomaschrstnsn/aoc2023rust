advent_of_code::solution!(3);

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct PartNumber {
    number: u32,
    start_pos: Coord,
    end_pos: Coord
}

#[derive(Debug, PartialEq, Eq)]
pub enum Entity {
    Symbol{position: Coord},
    Part(PartNumber)
}

struct Entities(Vec<Entity>);

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum ParseError {
    NotInt,
}

impl FromStr for Entities {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entities : Vec<Entity> = Vec::new();
        let lines : Vec<&str> = s.split('\n').collect();
        let mut in_number = false;
        for y in 0..lines.len() {
            let line = lines[y];
            for x in 0..line.len() {
                let c = dbg!(line.chars().nth(x).unwrap());
                match (c, c.is_ascii_digit()) {
                   ('.', _) => {in_number = false;},
                   (_, false) => {in_number = false; entities.push(Entity::Symbol { position: (x,y) })}
                   (_, true) => {
                       if in_number == false {
                           let number_str : String = line[x..].chars().take_while(|c| c.is_ascii_digit()).collect();
                           dbg!(&number_str);
                           let number = number_str.parse::<u32>().map_err(|_| ParseError::NotInt)?;
                           let ent = Entity::Part (PartNumber { number, start_pos: (x,y), end_pos: (x+number_str.len() - 1, y) });

                           entities.push(ent);
                           in_number = true;
                       }
                   }
                }
            }
        }
        Ok(Entities(entities))
    }
}

use std::collections::HashSet;

pub fn is_adjacent_to_one(&PartNumber{start_pos, end_pos, ..}: &PartNumber, &(sym_x, sym_y): &Coord) -> bool {
    let (sx,sy) = start_pos;
    let (ex,_ey) = end_pos;

    let dy = sym_y.abs_diff(sy);
    if dy < 2 {
        let dx1 = sym_x.abs_diff(sx);
        let dx2 = sym_x.abs_diff(ex);

        dx1 < 2 || dx2 < 2 || (ex <= sym_x || sx >= sym_x)

    } else {
        false
    }
}
pub fn is_adjacent_to_any(&PartNumber{start_pos, end_pos, ..}: &PartNumber, symbols: HashSet<&Coord>) -> bool {
    todo!();
}

pub fn part_one(input: &str) -> Option<u32> {
    let Entities(entities) = input.parse::<Entities>().ok()?;

    let symbols : HashSet<&Coord> = entities.iter().map(|e| {
        if let Entity::Symbol { position } = e {
            Some(position)
        }
        else {
            None
        }
    }).flatten().collect();

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let Entities(result) = input.parse::<Entities>().expect("should parse");
        let expected = vec!(
            Entity::Part (PartNumber { number: 467, start_pos: (0,0), end_pos: (2,0) }),
            Entity::Part (PartNumber { number: 114, start_pos: (5,0), end_pos: (7,0) }),
            Entity::Symbol { position: (3,1) },
            Entity::Part (PartNumber { number: 35, start_pos: (2,2), end_pos: (3,2) }),
            Entity::Part (PartNumber { number: 633, start_pos: (6,2), end_pos: (8,2) }),
            Entity::Symbol { position: (6,3) });

        dbg!(&result);
        assert_eq!(result.len(), 16);

        assert_eq!(result[..6], expected);
    }
}
