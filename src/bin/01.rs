use std::collections::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.split('\n') {
        let digits = line.chars()
            .filter(|c| c.is_ascii_digit())
            .collect::<Vec<char>>();

        let c1 = digits.first();
        let c2 = digits.last();

        if c1 == None {
            continue;
        }

        let s = format!("{}{}", c1?, c2?);

        let n = s.parse::<u32>();

        sum += n.unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let matchers = HashSet::from([
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let mut sum = 0;
    for line in input.split('\n') {
        let mut possible_digits = Vec::from_iter((0..line.len()).map(|_| None));
        for i in 0..line.len() {
            for (s, val) in matchers.iter() {
                if line[i..].starts_with(s) {
                    possible_digits[i] = Some(*val);
                    break;
                }
            }
        }

        let digits = possible_digits.iter().flatten().collect::<Vec<&u32>>();
        let c1 = digits.first();
        let c2 = digits.last();

        if c1 == None {
            continue;
        }
        let s = format!("{}{}", c1?, c2?);

        let n = s.parse::<u32>();

        sum += n.unwrap();
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let result = part_two(&input);
        assert_eq!(result, Some(281));
    }
}
