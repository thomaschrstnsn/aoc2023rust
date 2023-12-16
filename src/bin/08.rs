use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq, Ord, PartialOrd)]
struct NodeId(char, char, char);

impl NodeId {
    fn is_done(&self) -> bool {
        let NodeId(_, _, c) = self;
        *c == 'Z'
    }

    fn is_start(&self) -> bool {
        let NodeId(_, _, c) = self;
        *c == 'A'
    }
}

#[derive(Debug, PartialEq)]
struct Node {
    left: NodeId,
    right: NodeId,
}


#[derive(Debug)]
struct Input {
    directions: Vec<Direction>,
    graph: HashMap<NodeId, Node>,
}

#[derive(Debug, PartialEq, Eq)]
enum ParseError {
    DirectionInvalid(char),
    UnexpectedChar { expected: char },
    NodeIdTooLong(String),
    InvalidStructure,
}

fn parse(input: &str) -> Result<Input, ParseError> {
    use ParseError::*;
    let mut lines = input.lines();
    let directions_str = lines.next().ok_or(InvalidStructure)?;

    let directions: Vec<Direction> = directions_str
        .chars()
        .map(|c| match c {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            c => Err(DirectionInvalid(c)),
        })
        .collect::<Result<Vec<_>, ParseError>>()?;

    let blank = lines.next().ok_or(InvalidStructure)?;

    if !blank.is_empty() {
        return Err(InvalidStructure);
    }

    let graph: HashMap<NodeId, Node> = lines
        .map(parse_graph_entry)
        .collect::<Result<HashMap<NodeId, Node>, ParseError>>()?;

    Ok(Input { directions, graph })
}

fn parse_graph_entry(line: &str) -> Result<(NodeId, Node), ParseError> {
    use ParseError::*;

    let (node_id_str, node_str) = line.split_once('=').ok_or(InvalidStructure)?;

    let node_id = parse_node_id(node_id_str.trim())?;

    let (left_node_str, right_node_str) = node_str
        .trim()
        .strip_prefix('(')
        .ok_or(UnexpectedChar { expected: '(' })?
        .split_once(',')
        .ok_or(UnexpectedChar { expected: ',' })?;

    let left_node = parse_node_id(left_node_str.trim())?;
    let right_node = parse_node_id(&right_node_str.trim()[..3])?;

    Ok((
        node_id,
        Node {
            left: left_node,
            right: right_node,
        },
    ))
}

fn parse_node_id(s: &str) -> Result<NodeId, ParseError> {
    if s.len() != 3 {
        return Err(ParseError::NodeIdTooLong(s.to_owned()));
    }

    let mut chars = s.chars();
    Ok(NodeId(
        chars.next().unwrap(),
        chars.next().unwrap(),
        chars.next().unwrap(),
    ))
}

pub fn part_one(input: &str) -> Option<u32> {
    let input = parse(input).expect("should parse");

    let mut current_node: &NodeId = &parse_node_id("AAA").unwrap();

    let destination = parse_node_id("ZZZ").unwrap();

    let mut count = 0u32;
    for direction in input.directions.iter().cycle() {
        if *current_node == destination {
            break;
        }
        count += 1;
        let node = input.graph.get(current_node).expect("to find next node");
        current_node = match direction {
            Direction::Left => &node.left,
            Direction::Right => &node.right,
        };
    }

    Some(count)
}

#[derive(Debug)]
struct Recorder {
    period: Option<usize>,
}

impl Recorder {
    fn new() -> Recorder {
        Recorder { period: None}
    }

    fn record(&mut self, count: usize) {
        if self.period.is_none() {
            self.period = Some(count);
        }
    }

    fn is_done(&self) -> bool {
        self.period.is_some()
    }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn part_two(input: &str) -> Option<usize> {
    let input = parse(input).expect("should parse");

    let mut current_nodes: Vec<&NodeId> = input
        .graph
        .keys()
        .filter(|n| n.is_start())
        .collect();
    current_nodes.sort();

    let mut recordings : Vec<Recorder> = current_nodes.iter().map(|_| Recorder::new()).collect();

    let goal = current_nodes.len();

    for (count, direction) in input.directions.iter().cycle().enumerate() {
        let num_done = recordings
            .iter()
            .filter(|n| n.is_done())
            .count();

        for (i, node) in current_nodes.iter().enumerate() {
            if node.is_done() {
                recordings[i].record(count);
            }
        }

        if num_done == goal {
            break;
        }

        for current_node in current_nodes.iter_mut() {
            let node = input.graph.get(current_node).expect("to find next node");
            *current_node = match direction {
                Direction::Left => &node.left,
                Direction::Right => &node.right,
            };
        }
    }

    let divisors : Vec<usize> = recordings.iter().filter_map(|r| r.period).collect();

    Some(lcm(&divisors))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn node_id(s: &str) -> NodeId {
        parse_node_id(s).unwrap()
    }

    #[test]
    fn test_parse() {
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("it should parse");

        let expected_directions = vec![Direction::Left, Direction::Left, Direction::Right];
        let expected_graph = vec![
            (
                node_id("AAA"),
                Node {
                    left: node_id("BBB"),
                    right: node_id("BBB"),
                },
            ),
            (
                node_id("BBB"),
                Node {
                    left: node_id("AAA"),
                    right: node_id("ZZZ"),
                },
            ),
            (
                node_id("ZZZ"),
                Node {
                    left: node_id("ZZZ"),
                    right: node_id("ZZZ"),
                },
            ),
        ]
        .into_iter()
        .collect();

        assert_eq!(result.directions, expected_directions);
        assert_eq!(result.graph, expected_graph);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
