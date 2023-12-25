use std::{collections::VecDeque, num::ParseIntError};

use itertools::Itertools;

advent_of_code::solution!(15);

fn hash(input: &str) -> u32 {
    input
        .chars()
        .map(|c| c as u32)
        .fold(0, |acc, e| (acc + e) * 17 % 256)
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.trim().split(',').map(hash).sum())
}

type Label<'a> = &'a str;
type FocalLength = usize;

#[derive(Debug)]
enum Input<'a> {
    RemoveLens(Label<'a>),
    AddLens(Label<'a>, FocalLength),
}

impl<'a> Input<'a> {
    fn label(&self) -> &'a str {
        match self {
            Input::RemoveLens(l) => l,
            Input::AddLens(l, _) => l,
        }
    }
}

type Box<'a> = VecDeque<(Label<'a>, FocalLength)>;

enum ParseError {
    InvalidInput(char),
    InvalidInt(ParseIntError),
}

fn parse<'a>(s: &'a str) -> Result<Vec<Input<'a>>, ParseError> {
    s.split(',')
        .map(|e| {
            let label = &e[0..1];
            match e.chars().nth(2).unwrap() {
                '-' => Ok(Input::RemoveLens(&label)),
                '=' => {
                    let int_str = &e[2..];
                    let focal = int_str.parse::<usize>().map_err(ParseError::InvalidInt)?;
                    Ok(Input::AddLens(&label, focal))
                }
                invalid => Err(ParseError::InvalidInput(invalid)),
            }
        })
        .collect()
}

pub fn part_two(input: &str) -> Option<usize> {
    let inputs = parse(input).ok()?;

    let mut boxes: Vec<Box> = (0..256).map(|_| VecDeque::new()).collect();

    for input in inputs {
        dbg!(&input);
        let label = input.label();
        let box_index = hash(label) as usize;
        let the_box = boxes.get_mut(box_index).expect("can find the box");
        match input {
            Input::RemoveLens(_) => {
                if let Some(lens_index) = the_box.iter().position(|(l, _)| l == &label) {
                    the_box.remove(lens_index);
                }
            }
            Input::AddLens(label, focal) => {
                if let Some(lens_index) = the_box.iter().position(|(l, _)| l == &label) {
                    the_box.remove(lens_index);
                    the_box.insert(lens_index, (label, focal));
                } else {
                    the_box.push_back((label, focal));
                }
            }
        }
    }

    Some(
        boxes
            .iter()
            .enumerate()
            .map(|(box_index, lenses)| {
                lenses
                    .iter()
                    .enumerate()
                    .map(|(lens_index, (_, focal_length))| {
                        (box_index + 1) * (lens_index + 1) * focal_length
                    })
                    .sum::<usize>()
            })
            .sum::<usize>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn=1"), 30);
        assert_eq!(hash("pc=6"), 214);
    }
}
