use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<String>;

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Parsed {
    input.lines().map(|l| l.to_owned()).collect()
}

#[aoc(day3, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .map(|l| l.split_at(l.len() / 2))
        .map(|(a, b)| calc_priority(find_common(vec![a, b])))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .chunks(3)
        .map(|c| calc_priority(find_common(c.iter().map(|s| s.as_str()).collect())))
        .sum()
}

fn calc_priority(a: char) -> usize {
    if a.is_lowercase() {
        a as usize - ('a' as usize) + 1
    } else {
        a as usize - ('A' as usize) + 27
    }
}
fn find_common(s: Vec<&str>) -> char {
    let sets: Vec<HashSet<char>> = s.iter().skip(1).map(|s| s.chars().collect()).collect();

    s[0].chars()
        .find(|c| sets.iter().all(|s| s.contains(c)))
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 157);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 70);
    }
}
