use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(11);

type Coord = (usize, usize);

fn parse(s: &str) -> Vec<Coord> {
    s.lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some((x.clone(), y.clone()))
                } else {
                    None
                }
            })
        })
        .flatten()
        .collect()
}

#[derive(Debug)]
struct ExpansionLines {
    horizontal: Vec<usize>,
    vertical: Vec<usize>,
}

fn find_expansion_lines(galaxies: &[Coord]) -> ExpansionLines {
    let max_x = galaxies
        .iter()
        .max_by_key(|(x, _)| x)
        .expect("should have max x")
        .0;

    let verticals_occupied: HashSet<usize> = galaxies.iter().map(|(x, _)| x.clone()).collect();
    let verticals_all: HashSet<usize> = (0usize..max_x).collect();
    let mut verticals_free: Vec<usize> = verticals_all
        .difference(&verticals_occupied)
        .map(|&x| x.clone())
        .collect();
    verticals_free.sort();

    let max_y = galaxies
        .iter()
        .max_by_key(|(_, y)| y)
        .expect("should have max y")
        .1;

    let horizontals_occupied: HashSet<usize> = galaxies.iter().map(|(_, y)| y.clone()).collect();
    let horizontals_all: HashSet<usize> = (0usize..max_y).collect();
    let mut horizontals_free: Vec<usize> = horizontals_all
        .difference(&horizontals_occupied)
        .map(|&y| y.clone())
        .collect();
    horizontals_free.sort();

    ExpansionLines {
        horizontal: horizontals_free,
        vertical: verticals_free,
    }
}

fn expanded_universe(galaxies: &[Coord], expansion_lines: &ExpansionLines, expansion_factor: usize) -> Vec<Coord> {
    galaxies
        .iter()
        .map(|&(x, y)| {
            let vertical_expansions_before = expansion_lines
                .vertical
                .iter()
                .filter(|&vex| vex < &x)
                .count();
            let horizontal_expansions_before = expansion_lines
                .horizontal
                .iter()
                .filter(|&hex| hex < &y)
                .count();
            (
                x + (vertical_expansions_before * expansion_factor),
                y + (horizontal_expansions_before * expansion_factor),
            )
        })
        .collect()
}

fn solve(input: &str, factor: usize) -> Option<usize> {
    let galaxies = parse(input);

    let expansions = find_expansion_lines(&galaxies);

    let expanded_universe = expanded_universe(&galaxies, &expansions, factor);

    let result = expanded_universe.iter().tuple_combinations().map(|(a,b)| manhattan_dist(a,b)).sum();

    Some(result)
}

pub fn part_one(input: &str) -> Option<usize> {
    solve(input, 1usize)
}

fn manhattan_dist(a: &Coord, b: &Coord) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

pub fn part_two(input: &str) -> Option<usize> {
    solve(input, 1_000_000usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two_10() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 10usize);
        assert_eq!(result, Some(1030));
    }

    #[test]
    fn test_part_two_100() {
        let result = solve(&advent_of_code::template::read_file("examples", DAY), 100usize);
        assert_eq!(result, Some(8410));
    }

    #[test]
    fn test_parse_example() {
        let result = parse(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result.len(), 9);
        assert_eq!(result.iter().next(), Some(&(3usize, 0usize)));
    }
}
