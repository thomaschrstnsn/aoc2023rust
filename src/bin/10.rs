use std::collections::{HashMap, HashSet};

use itertools::Itertools;

advent_of_code::solution!(10);

#[derive(Debug, Eq, PartialEq)]
enum Point {
    Start,
    WE,
    NS,
    NW,
    SW,
    NE,
    SE,
}

#[derive(Debug, Hash, Eq, PartialEq)]
enum Direction {
    North,
    West,
    South,
    East,
}

impl Point {
    fn is_connected_to(&self, direction: &Direction) -> bool {
        use Direction::*;
        use Point::*;

        matches!(
            (self, direction),
            (Start, _)
                | (WE, West)
                | (WE, East)
                | (NS, North)
                | (NS, South)
                | (NW, North)
                | (NW, West)
                | (SW, South)
                | (SW, West)
                | (SE, South)
                | (SE, East)
                | (NE, North)
                | (NE, East)
        )
    }
}

type Row = Vec<Option<Point>>;

struct Input(Vec<Row>);

type Coord = (isize, isize);

impl Input {
    fn new(height: usize, width: usize) -> Input {
        let value: Vec<Row> = (0usize..height)
            .map(|_| {
                let mut v = Vec::with_capacity(width);
                for _ in 0..width {
                    v.push(None);
                }
                v
            })
            .collect();
        Input(value)
    }

    fn get_row_mut(&mut self, index: usize) -> &mut Row {
        self.0.get_mut(index).unwrap()
    }

    fn get_row(&self, index: usize) -> Option<&Row> {
        self.0.get(index)
    }

    fn get_point(&self, (x, y): Coord) -> Option<&Option<Point>> {
        let x: usize = x.try_into().ok()?;
        let y: usize = y.try_into().ok()?;

        self.get_row(y)?.get(x)
    }

    fn filter(&self, pred: fn(&Option<Point>) -> bool) -> Vec<Coord> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter().enumerate().filter_map(move |(x, point)| {
                    if pred(point) {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }

    fn get_immediate_connections(&self, (x, y): Coord) -> Vec<Coord> {
        use Direction::*;
        if let Some(Some(point)) = self.get_point((x, y)) {
            let possible_directions: HashSet<Direction> = match point {
                Point::NS => vec![North, South],
                Point::WE => vec![West, East],
                Point::NW => vec![North, West],
                Point::Start => vec![North, East, South, West],
                Point::SW => vec![South, West],
                Point::NE => vec![North, East],
                Point::SE => vec![South, East],
            }
            .into_iter()
            .collect();
            vec![
                ((0, 1isize), North, South),
                ((0, -1isize), South, North),
                ((1, 0), West, East),
                ((-1isize, 0), East, West),
            ]
            .into_iter()
            .filter(|(_, _, d)| possible_directions.contains(d))
            .filter_map(|((dx, dy), incoming_direction, _outgoing_direction)| {
                let next_coord = (x + dx, y + dy);
                let next_point = self.get_point(next_coord)?;
                if let Some(next_point) = next_point {
                    if next_point.is_connected_to(&incoming_direction) {
                        return Some(next_coord);
                    }
                }
                None
            })
            .collect()
        } else {
            Vec::new()
        }
    }

    fn connections_from(&self, c: Coord) -> HashSet<Coord> {
        let mut visited: HashSet<Coord> = HashSet::new();
        visited.insert(c);

        let mut connections: HashSet<Coord> =
            self.get_immediate_connections(c).into_iter().collect();
        while !connections.is_empty() {
            visited.extend(&connections);

            let new_cons: HashSet<Coord> = connections
                .iter()
                .flat_map(|&c| self.get_immediate_connections(c))
                .collect();
            connections = new_cons.difference(&visited).cloned().collect();
        }

        visited
    }

    fn counter_clockwise_connections_from(&self, c: Coord) -> Vec<Coord> {
        let mut visited: HashSet<Coord> = HashSet::new();
        visited.insert(c);

        let mut result: Vec<_> = vec![c];

        let mut connections: HashSet<Coord> = self
            .get_immediate_connections(c)
            .into_iter()
            .sorted()
            .take(1)
            .collect();
        // .filter(|(x, y)| (c.0 > *x && c.1 <= *y) || (c.0 >= *x && c.1 < *y))
        // .collect();
        while !connections.is_empty() {
            visited.extend(&connections);

            assert_eq!(connections.len(), 1, "{:?}", connections);
            let c = connections.iter().next().unwrap();
            if let Some(Some(point)) = self.get_point(*c) {
                if !matches!(point, Point::NS | Point::WE) {
                    result.push(*c);
                }
            }

            let new_cons: HashSet<Coord> = connections
                .iter()
                .flat_map(|&c| self.get_immediate_connections(c))
                .collect();
            connections = new_cons.difference(&visited).cloned().collect();
        }

        result
    }

    fn furthest_connection_from(&self, c: Coord) -> usize {
        let mut visited: HashSet<Coord> = HashSet::new();
        visited.insert(c);

        let mut distance = 0usize;
        let mut connections: HashSet<Coord> =
            self.get_immediate_connections(c).into_iter().collect();
        while !connections.is_empty() {
            visited.extend(&connections);

            let new_cons: HashSet<Coord> = connections
                .iter()
                .flat_map(|&c| self.get_immediate_connections(c))
                .collect();
            connections = new_cons.difference(&visited).cloned().collect();
            distance += 1;
        }

        distance
    }

    fn print(&self) -> String {
        self.0
            .iter()
            .map(|row| row.iter().map(point_to_char).collect::<String>())
            .join("\n")
    }

    fn print_filtered(&self, filter_coords: &HashSet<Coord>) -> String {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, p)| {
                        if filter_coords.contains(&(x as isize, y as isize)) {
                            point_to_char(p)
                        } else {
                            'x'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }

    fn print_filtered_with_numbered_overlay(
        &self,
        filter_coords: &HashSet<Coord>,
        overlay: &[Coord],
    ) -> String {
        let lookup: HashMap<&Coord, usize> =
            overlay.iter().enumerate().map(|(i, c)| (c, i)).collect();
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, p)| {
                        let c = (x as isize, y as isize);
                        if let Some(index) = lookup.get(&c) {
                            format!("{}", index % 10).chars().next().unwrap()
                        } else if filter_coords.contains(&c) {
                            point_to_char(p)
                        } else {
                            '·'
                        }
                    })
                    .collect::<String>()
            })
            .join("\n")
    }
}

#[derive(Debug)]
pub enum ParseError {
    UnexpectedChar(char),
    InvalidLineLength { expected: usize, found: usize },
}

fn parse(s: &str) -> Result<Input, ParseError> {
    let width = s.lines().next().expect("atleast one line").len();
    let height = s.lines().count();

    let mut output = Input::new(height, width);

    for (index, line) in s.lines().enumerate() {
        let row = output.get_row_mut(index); // output.get_mut(index).unwrap();
        parse_line_into_row(line, row, width)?;
    }

    Ok(output)
}

fn parse_line_into_row(
    line: &str,
    row: &mut Row,
    expected_length: usize,
) -> Result<(), ParseError> {
    if line.len() != expected_length {
        return Err(ParseError::InvalidLineLength {
            expected: expected_length,
            found: line.len(),
        });
    }
    for (c, point) in std::iter::zip(line.chars(), row.iter_mut()) {
        *point = parse_point(c)?;
    }
    Ok(())
}

fn point_to_char(p: &Option<Point>) -> char {
    if let Some(point) = p {
        match point {
            Point::Start => 'S',
            Point::WE => '─',
            Point::NS => '│',
            Point::NW => '┘',
            Point::SW => '┐',
            Point::NE => '└',
            Point::SE => '┌',
        }
    } else {
        '╳'
    }
}

fn parse_point(c: char) -> Result<Option<Point>, ParseError> {
    use Point::*;
    match c {
        'S' => Ok(Some(Start)),
        '|' => Ok(Some(NS)),
        '-' => Ok(Some(WE)),
        'L' => Ok(Some(NE)),
        'J' => Ok(Some(NW)),
        '7' => Ok(Some(SW)),
        'F' => Ok(Some(SE)),
        '.' => Ok(None),
        unexpected => Err(ParseError::UnexpectedChar(unexpected)),
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let input = parse(input).expect("should parse");

    let binding = input.filter(|x| *x == Some(Point::Start));
    let start = binding.first().expect("should have a start point");

    Some(input.furthest_connection_from(*start))
}

pub fn shoelace(vertices: &[Coord]) -> isize {
    isize::abs(
        vertices
            .iter()
            .circular_tuple_windows()
            .map(|((x0, y0), (x1, y1))| x0 * y1 - (x1 * y0))
            .sum::<isize>(),
    ) / 2
}

pub fn part_two(input: &str) -> Option<isize> {
    let input = parse(input).expect("should parse");

    println!("input:\n{}", input.print());

    let binding = input.filter(|x| *x == Some(Point::Start));
    let start = binding.first().expect("should have a start point");

    let connections = input.connections_from(*start);
    let vertices = input.counter_clockwise_connections_from(*start);

    println!("loop:\n{}", input.print_filtered(&connections));
    // println!(
    //     "{}",
    //     input.print_filtered_with_numbered_overlay(&connections, &vertices)
    // );

    Some(shoelace(&vertices) - (&connections.len() / 2) as isize + 1)
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
    fn test_part_two_example_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two_example_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two_example_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_parse_example() {
        use Point::*;
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("should parse");

        let expected_first: Row = vec![SW, WE, SE, SW, WE]
            .into_iter()
            .map(|e| Some(e))
            .collect();

        assert_eq!(result.get_row(0), Some(&expected_first));
    }

    #[test]
    fn test_shoelace() {
        let result = shoelace(&vec![(2, 1), (5, 0), (6, 4), (4, 2), (1, 3)]);
        assert_eq!(result, 8);
    }
}
