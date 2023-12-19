use std::collections::HashSet;

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
            let directions : HashSet<Direction> = match point {
                Point::NS => vec![North, South],
                Point::WE => vec![West, East],
                Point::NW => vec![South, East],
                Point::Start => vec![North, East, South, West],
                Point::SW => vec![North, East],
                Point::NE => vec![South, West],
                Point::SE => vec![North, West],
            }.into_iter().collect();
            vec![
                ((0, 1isize), North),
                ((0, -1isize), South),
                ((1, 0), West),
                ((-1isize, 0), East),
            ]
            .into_iter()
            .filter(|(_, d)| directions.contains(d))
            .filter_map(|((dx, dy), direction)| {
                let next_coord = (x + dx, y + dy);
                let point = self.get_point(next_coord)?;
                if let Some(point) = point {
                    if point.is_connected_to(&direction) {
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

    fn print_filtered_with_overlay(&self, filter_coords: &HashSet<Coord>, overlay: &HashSet<Coord>, oc: char) -> String {
        self.0
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, p)| {
                        let c = (x as isize, y as isize);
                        if overlay.contains(&c) {
                            oc
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

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input).expect("should parse");

    println!("{}", input.print());

    let binding = input.filter(|x| *x == Some(Point::Start));
    let start = binding.first().expect("should have a start point");

    let connections = input.connections_from(*start);

    println!("{}", input.print_filtered(&connections));

    let min_x = connections
        .iter()
        .min_by_key(|(x, _)| x)
        .expect("have a minimum")
        .0;
    let max_x = connections
        .iter()
        .max_by_key(|(x, _)| x)
        .expect("have a maximum")
        .0;
    let min_y = connections
        .iter()
        .min_by_key(|(_, y)| y)
        .expect("have a minimum")
        .1;
    let max_y = connections
        .iter()
        .max_by_key(|(_, y)| y)
        .expect("have a maximum")
        .1;

    let mut horizontal_insiders: HashSet<Coord> = HashSet::new();
    for y in min_y..max_y {
        let mut inside = false;
        for x in min_x..max_x {
            let is_connected = connections.contains(&(x, y));
            if is_connected {
                if let Some(Some(point)) = input.get_point((x,y)) {
                    if *point == Point::WE {
                        inside = inside;
                    } else if *point != Point::WE {
                        inside = !inside;
                    }
                }
            } else if inside {
                horizontal_insiders.insert((x, y));
            }
        }
    }
    let mut vertical_insiders: HashSet<Coord> = HashSet::new();
    for x in min_x..max_x {
        let mut inside = false;
        for y in min_y..max_y {
            let is_connected = connections.contains(&(x, y));
            if is_connected {
                if let Some(Some(point)) = input.get_point((x,y)) {
                    if *point == Point::NS {
                        inside = inside;
                    } else if *point != Point::NS {
                        inside = !inside;
                    }
                }
            } else if inside {
                // dbg!((x,y));
                vertical_insiders.insert((x, y));
            }
        }
    }
    // dbg!(&vertical_insiders);
    // dbg!(&horizontal_insiders);
    println!("HOR:\n{}", input.print_filtered_with_overlay(&connections, &horizontal_insiders, 'H'));
    println!("VER:\n{}", input.print_filtered_with_overlay(&connections, &vertical_insiders, 'V'));

    let inside_area = horizontal_insiders.intersection(&vertical_insiders).count();
    // let inside_area = horizontal_insiders.len();

    Some(inside_area)
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
}
