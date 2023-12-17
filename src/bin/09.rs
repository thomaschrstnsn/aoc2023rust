use std::num::ParseIntError;

use itertools::Itertools;

advent_of_code::solution!(9);

type Input = Vec<i64>;

fn parse(s: &str) -> Result<Vec<Input>, ParseIntError> {
    s.lines().map(parse_single_input).collect()
}

fn parse_single_input(s: &str) -> Result<Input, ParseIntError> {
    s.split(' ').map(|s| s.parse::<i64>()).collect()
}

fn extrapolate_next_value(i: &Input) -> i64 {
    let mut sequences : Vec<Box<Vec<i64>>> = vec![Box::new(i.clone())];
    let mut done = false;
    while !done {
        let current = sequences.last().expect("should have a last");
        let next : Vec<i64> = current.iter().tuple_windows().map(|(x,y)| y - x).collect();

        done = next.iter().all(|x| *x == 0);
        sequences.push(Box::new(next));
    }

    let result = sequences.iter().rev().fold(0, |prev, sequence| {
        let last = sequence.last().unwrap();
        last + prev
    });

    result
}

pub fn part_one(input: &str) -> Option<i64> {
    let inputs = parse(input).expect("it should parse");

    Some(inputs.iter().map(extrapolate_next_value).sum())
}

fn extrapolate_first_value(i: &Input) -> i64 {
    let mut sequences : Vec<Box<Vec<i64>>> = vec![Box::new(i.clone())];
    let mut done = false;
    while !done {
        let current = sequences.last().expect("should have a last");
        let next : Vec<i64> = current.iter().tuple_windows().map(|(x,y)| y - x).collect();

        done = next.iter().all(|x| *x == 0);
        sequences.push(Box::new(next));
    }

    let result = sequences.iter().rev().fold(0, |prev, sequence| {
        let first = sequence.first().unwrap();
        first - prev
    });

    result
}

pub fn part_two(input: &str) -> Option<i64> {
    let inputs = parse(input).expect("it should parse");

    Some(inputs.iter().map(extrapolate_first_value).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_parse_example() {
        let result = parse(&advent_of_code::template::read_file("examples", DAY)).expect("it should parse");
        assert_eq!(result, vec![vec![0,3,6,9,12,15], vec![1,3,6,10,15,21], vec![10,13,16,21,30,45]])
    }

    #[test]
    fn test_negative() {
        let result = parse_single_input("-12 12 24 -24 -1000").expect("it should parse");
        assert_eq!(result, vec![-12, 12, 24, -24, -1000])
    }
}
