use std::{collections::HashMap, num::ParseIntError};

advent_of_code::solution!(19);

type Value = u64;

#[derive(Debug, Eq, PartialEq)]
struct Part {
    x: Value,
    m: Value,
    a: Value,
    s: Value,
}

#[derive(Debug, Eq, PartialEq)]
enum Quality {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Eq, PartialEq)]
enum Condition {
    LessThan,
    GreaterThan,
}

#[derive(Debug, Eq, PartialEq)]
struct Rule<'a> {
    qual: Quality,
    cond: Condition,
    val: Value,
    destination: &'a str,
}

struct Input<'a> {
    rules: HashMap<&'a str, Vec<Rule<'a>>>,
    parts: Vec<Part>,
}

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    Expected(char),
    Unmatched(char),
    InvalidInt(ParseIntError),
}

fn parse(s: &str) -> Result<Input, ParseError> {
    let mut rules: HashMap<_, _> = HashMap::new();
    let mut parts = Vec::new();
    let mut doing_parts = false;
    for line in s.lines() {
        if line.trim().is_empty() {
            doing_parts = true;
            continue;
        }
        if !doing_parts {
            let (name, rule) = parse_rules(line)?;
            rules.insert(name, rule);
        } else {
            let part = parse_part(line)?;
            parts.push(part);
        }
    }

    Ok(Input { rules, parts })
}

fn parse_part_quality(input: &str, quality: char) -> Result<Value, ParseError> {
    let (actual_qual, value_str) = input.split_once('=').ok_or(ParseError::Expected('='))?;

    if actual_qual.len() > 1 || actual_qual.chars().next().unwrap() != quality {
        return Err(ParseError::Expected(quality));
    }

    let value = value_str.parse::<Value>().map_err(ParseError::InvalidInt)?;

    Ok(value)
}

fn parse_part(line: &str) -> Result<Part, ParseError> {
    let inner = line.strip_prefix('{').ok_or(ParseError::Expected('{'))?;
    let inner = inner.strip_suffix('}').ok_or(ParseError::Unmatched('{'))?;

    let mut qualities = inner.split(',');
    let x_str = qualities.next().ok_or(ParseError::Expected(','))?;
    let x = parse_part_quality(x_str, 'x')?;

    let m_str = qualities.next().ok_or(ParseError::Expected(','))?;
    let m = parse_part_quality(m_str, 'm')?;

    let a_str = qualities.next().ok_or(ParseError::Expected(','))?;
    let a = parse_part_quality(a_str, 'a')?;

    let s_str = qualities.next().ok_or(ParseError::Expected(','))?;
    let s = parse_part_quality(s_str, 's')?;

    Ok(Part { x, m, a, s })
}

fn parse_rules(line: &str) -> Result<(&str, Vec<Rule>), ParseError> {
    todo!()
}

pub fn part_one(input: &str) -> Option<u32> {
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
