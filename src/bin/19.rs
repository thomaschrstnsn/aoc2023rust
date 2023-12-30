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

impl Part {
    fn combined(&self) -> Value {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Quality {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Eq, PartialEq)]
struct Workflow<'a> {
    name: &'a str,
    rules: Vec<Rule<'a>>,
    default_destination: &'a str,
}

impl<'a> Workflow<'a> {
    fn destination_for_part(&self, part: &Part) -> &'a str {
        let rule_match = self
            .rules
            .iter()
            .filter_map(|r| {
                if r.matches(part) {
                    Some(r.destination)
                } else {
                    None
                }
            })
            .next();
        if let Some(dest) = rule_match {
            dest
        } else {
            self.default_destination
        }
    }
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

impl<'a> Rule<'a> {
    fn matches(&self, part: &Part) -> bool {
        let actual_val = match self.qual {
            Quality::X => part.x,
            Quality::M => part.m,
            Quality::A => part.a,
            Quality::S => part.s,
        };

        match self.cond {
            Condition::LessThan => actual_val < self.val,
            Condition::GreaterThan => actual_val > self.val,
        }
    }
}

struct Input<'a> {
    workflows: HashMap<&'a str, Workflow<'a>>,
    parts: Vec<Part>,
}

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    Expected(char),
    Unmatched(char),
    InvalidInt(ParseIntError),
    ExpectedDefaultDestination,
}

fn parse(s: &str) -> Result<Input, ParseError> {
    let mut workflows: HashMap<_, _> = HashMap::new();
    let mut parts = Vec::new();
    let mut doing_parts = false;
    for line in s.lines() {
        if line.trim().is_empty() {
            doing_parts = true;
            continue;
        }
        if !doing_parts {
            let workflow = parse_workflow(line)?;
            workflows.insert(workflow.name, workflow);
        } else {
            let part = parse_part(line)?;
            parts.push(part);
        }
    }

    Ok(Input { workflows, parts })
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

fn parse_rule(input: &str) -> Result<Rule, ParseError> {
    let (qual_cond_str, destination) = input.split_once(':').ok_or(ParseError::Expected(':'))?;

    let mut qual_cond_iter = qual_cond_str.chars();

    let qual = match qual_cond_iter.next().ok_or(ParseError::Expected('q'))? {
        'x' => Ok(Quality::X),
        'm' => Ok(Quality::M),
        'a' => Ok(Quality::A),
        's' => Ok(Quality::S),
        _ => Err(ParseError::Expected('q')),
    }?;

    let cond = match qual_cond_iter.next().ok_or(ParseError::Expected('c'))? {
        '>' => Ok(Condition::GreaterThan),
        '<' => Ok(Condition::LessThan),
        _ => Err(ParseError::Expected('c')),
    }?;

    let value_str: String = qual_cond_iter.collect();
    let val = value_str.parse::<Value>().map_err(ParseError::InvalidInt)?;

    Ok(Rule {
        qual,
        cond,
        val,
        destination,
    })
}

fn parse_workflow(line: &str) -> Result<Workflow, ParseError> {
    let (name, rest) = line.split_once('{').ok_or(ParseError::Expected('{'))?;

    let inner = rest.strip_suffix('}').ok_or(ParseError::Expected('}'))?;

    let mut rules: Vec<_> = Vec::new();
    let mut destination: Option<&str> = None;
    for rule_str in inner.split(',') {
        let rule_res = parse_rule(rule_str);
        match rule_res {
            Err(e) => {
                if destination.is_none() {
                    destination = Some(rule_str);
                } else {
                    return Err(e);
                }
            }
            Ok(rule) => {
                rules.push(rule);
            }
        }
    }

    Ok(Workflow {
        name,
        rules,
        default_destination: destination.ok_or(ParseError::ExpectedDefaultDestination)?,
    })
}

fn run_part_through_workflows<'a>(
    part: &Part,
    workflows: &HashMap<&str, Workflow<'a>>,
    start: &'a str,
    ends: &[&'a str],
) -> &'a str {
    let mut cur = start;
    while !ends.contains(&cur) {
        let wf = workflows.get(cur).unwrap();
        cur = wf.destination_for_part(part);
    }

    cur
}

pub fn part_one(input: &str) -> Option<Value> {
    let input = parse(input).expect("should parse");

    let start = "in";
    let accepted = "A";
    let ends = vec![accepted, "R"];

    let accepted_parts: Vec<_> = input
        .parts
        .iter()
        .filter(|part| run_part_through_workflows(part, &input.workflows, start, &ends) == accepted)
        .collect();

    Some(accepted_parts.iter().map(|p| p.combined()).sum())
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_parse_example() {
        let input = &advent_of_code::template::read_file("examples", DAY);
        let result = parse(input).expect("a succesfull parse");

        let first_workflow = Workflow {
            name: "px",
            rules: vec![
                Rule {
                    qual: Quality::A,
                    cond: Condition::LessThan,
                    val: 2006,
                    destination: "qkq",
                },
                Rule {
                    qual: Quality::M,
                    cond: Condition::GreaterThan,
                    val: 2090,
                    destination: "A",
                },
            ],
            default_destination: "rfg",
        };

        assert_eq!(result.workflows.get("px"), Some(&first_workflow));

        let first_part = Part {
            x: 787,
            m: 2655,
            a: 1222,
            s: 2876,
        };

        assert_eq!(result.parts[0], first_part);
    }
}
