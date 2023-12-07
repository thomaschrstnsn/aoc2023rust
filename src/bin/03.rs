advent_of_code::solution!(3);

type Coord = (usize, usize);

#[derive(Debug, PartialEq, Eq)]
pub struct PartNumber {
    number: u32,
    start_pos: Coord,
    end_pos: Coord,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Entity {
    Symbol{position: Coord, symbol: char},
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
        for y in 0..lines.len() {
            let line = lines[y];
            let mut in_number = false;
            for x in 0..line.len() {
                let c = line.chars().nth(x).unwrap();
                match (c, c.is_ascii_digit()) {
                   ('.', _) => {in_number = false;},
                   (c, false) => {in_number = false; entities.push(Entity::Symbol { position: (x,y), symbol: c })}
                   (_, true) => {
                       if in_number == false {
                           let number_str : String = line[x..].chars().take_while(|c| c.is_ascii_digit()).collect();
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

pub fn is_adjacent_to_one(&PartNumber{start_pos, end_pos, ..}: &PartNumber, &(sym_x, sym_y): &Coord) -> bool {
    let (sx,sy) = start_pos;
    let (ex,_ey) = end_pos;

    let dy = sym_y.abs_diff(sy);
    if dy < 2 {
        let dx1 = sym_x.abs_diff(sx);
        let dx2 = sym_x.abs_diff(ex);

        dx1 < 2 || dx2 < 2 || (ex <= sym_x && sx >= sym_x)

    } else {
        false
    }
}

pub fn is_adjacent_to_any(pn: &PartNumber, symbols: &Vec<&Coord>) -> bool {
    symbols.iter().any(|&s| is_adjacent_to_one(&pn, s))
}

pub fn part_one(input: &str) -> Option<u32> {
    let Entities(entities) = input.parse::<Entities>().ok()?;

    let symbols : Vec<&Coord> = entities.iter().map(|e| {
        if let Entity::Symbol { position, symbol: _} = e {
            Some(position)
        }
        else {
            None
        }
    }).flatten().collect();

    Some(entities.iter().map(
            |e| {
                if let Entity::Part(partnumber) = e {
                    Some(partnumber)
                } else {
                    None
                }
            })
            .flatten().filter(|&pn| is_adjacent_to_any(pn, &symbols))
        .map(|e| e.number)
        .sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    let Entities(entities) = input.parse::<Entities>().ok()?;

    let gears : Vec<&Coord> = entities.iter().map(|e| {
        if let Entity::Symbol { position, symbol: '*'} = e {
            Some(position)
        }
        else {
            None
        }
    }).flatten().collect();

    dbg!(gears);
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
            Entity::Symbol { position: (3,1), symbol: '*' },
            Entity::Part (PartNumber { number: 35, start_pos: (2,2), end_pos: (3,2) }),
            Entity::Part (PartNumber { number: 633, start_pos: (6,2), end_pos: (8,2) }),
            Entity::Symbol { position: (6,3), symbol: '#' });

        assert_eq!(result.len(), 16);

        assert_eq!(result[..6], expected);
    }

    #[test]
    fn test_is_adjacent_to_one() {
        let part_number = PartNumber { number: 123, start_pos: (5,6), end_pos: (7,6)};

        assert!(is_adjacent_to_one(&part_number, &(4,5)));
        assert!(is_adjacent_to_one(&part_number, &(4,6)));
        assert!(is_adjacent_to_one(&part_number, &(4,7)));

        assert!(is_adjacent_to_one(&part_number, &(6,5)));
        assert!(is_adjacent_to_one(&part_number, &(6,7)));

        assert!(is_adjacent_to_one(&part_number, &(5,5)));
        assert!(is_adjacent_to_one(&part_number, &(5,7)));

        assert!(is_adjacent_to_one(&part_number, &(7,5)));
        assert!(is_adjacent_to_one(&part_number, &(7,7)));

        assert!(is_adjacent_to_one(&part_number, &(8,7)));
        assert!(is_adjacent_to_one(&part_number, &(8,6)));
        assert!(is_adjacent_to_one(&part_number, &(8,5)));

        assert!(!is_adjacent_to_one(&part_number, &(9,7)));
        assert!(!is_adjacent_to_one(&part_number, &(9,6)));
        assert!(!is_adjacent_to_one(&part_number, &(9,5)));

        assert!(!is_adjacent_to_one(&part_number, &(3,7)));
        assert!(!is_adjacent_to_one(&part_number, &(3,6)));
        assert!(!is_adjacent_to_one(&part_number, &(3,5)));
    }

    #[test]
    fn test_is_adjacent_to_one_2() {
        let symbol : Coord = ( 5, 5,);

        let not_adjacent =  PartNumber { number: 58, start_pos: ( 7, 5,), end_pos: ( 8, 5,), };

        let adjacent = PartNumber { number: 592, start_pos: ( 2, 6,), end_pos: ( 4, 6,), };

        assert!(is_adjacent_to_one(&adjacent, &symbol));
        assert!(!is_adjacent_to_one(&not_adjacent, &symbol));
    }
}
