use std::collections::HashMap;

advent_of_code::solution!(8);

#[derive(Debug, PartialEq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct NodeId(char, char, char);

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

pub fn part_two(input: &str) -> Option<u32> {
    None
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
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
