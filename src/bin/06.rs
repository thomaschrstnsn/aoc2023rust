advent_of_code::solution!(6);

#[derive(Debug, PartialEq)]
struct Races {
    time: Vec<u32>,
    distance: Vec<u32>,
}

fn parse(input: &str) -> Option<Races> {
    let lines: Vec<&str> = input.lines().collect();

    assert_eq!(lines.len(), 2);

    let time_str = lines[0].strip_prefix("Time:")?.trim();
    let distance_str = lines[1].strip_prefix("Distance:")?.trim();

    let time = u32s(time_str).ok()?;
    let distance = u32s(distance_str).ok()?;

    assert_eq!(time.len(), distance.len());

    Some(Races { time, distance })
}

fn u32s(s: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    s.trim()
        .split(' ')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().parse::<u32>())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input).expect("parses");
    let mut result = 1;
    for (time, distance) in std::iter::zip(races.time, races.distance) {
        let hold_times: Vec<u32> = (1..time)
            .filter(|hold_time| (time - hold_time) * hold_time > distance)
            .collect();

        let min = hold_times.iter().min().expect("to have minimum");
        let max = hold_times.iter().max().expect("to have maximum");
        let delta = max - min + 1;
        result *= delta;
    }

    Some(result)
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
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }

    #[test]
    fn parse_example() {
        let result =
            parse(&advent_of_code::template::read_file("examples", DAY)).expect("should parse");

        let expected = Races {
            time: vec![7, 15, 30],
            distance: vec![9, 40, 200],
        };

        assert_eq!(result, expected);
    }
}
