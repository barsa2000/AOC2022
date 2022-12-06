use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

type Parsed = Vec<char>;

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Parsed {
    input.chars().collect()
}

#[aoc(day6, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .windows(4)
        .find_position(|s| s.iter().all_unique())
        .unwrap()
        .0
        + 4
}

#[aoc(day6, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .windows(14)
        .find_position(|s| s.iter().all_unique())
        .unwrap()
        .0
        + 14
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 7);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 19);
    }
}
