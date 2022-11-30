use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<u128>;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Parsed {
    todo!()
}

#[aoc(day1, part1)]
fn part1(input: &Parsed) -> usize {
    todo!()
}

#[aoc(day1, part2)]
fn part2(input: &Parsed) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        ""
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 0);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 0);
    }
}
