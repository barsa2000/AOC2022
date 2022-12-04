use aoc_runner_derive::{aoc, aoc_generator};
use scan_fmt::scan_fmt;

type Parsed = Vec<((u128, u128), (u128, u128))>;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let (a0, a1, b0, b1) = scan_fmt!(l, "{}-{},{}-{}", u128, u128, u128, u128).unwrap();
            ((a0, a1), (b0, b1))
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .filter(|(a, b)| (a.0 >= b.0 && a.1 <= b.1) || (b.0 >= a.0 && b.1 <= a.1))
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .iter()
        .filter(|(a, b)| !(a.0 > b.1 || a.1 < b.0) || (b.0 <= a.1 && b.1 >= a.0))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 2);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 4);
    }
}
