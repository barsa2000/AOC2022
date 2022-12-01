use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<Vec<u128>>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Parsed {
    input
        .split("\n\n")
        .map(|e| e.lines().map(|l| l.parse::<u128>().unwrap()).collect())
        .collect()
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> u128 {
    input.iter().map(|e| e.iter().sum()).max().unwrap()
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> u128 {
    let mut sorted: Vec<u128> = input.iter().map(|e| e.iter().sum()).collect();
    sorted.sort_unstable();
    sorted.iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 24000);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 45000);
    }
}
