use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<(Vec<u128>, Vec<u128>)>;

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let mut elves = l.split(',');
            let a = elves.next().unwrap();
            let b = elves.next().unwrap();

            let a: Vec<&str> = a.split('-').collect();
            let b: Vec<&str> = b.split('-').collect();

            (
                (a[0].parse().unwrap()..a[1].parse::<u128>().unwrap() + 1_u128).collect(),
                (b[0].parse().unwrap()..b[1].parse::<u128>().unwrap() + 1_u128).collect(),
            )
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(input: &Parsed) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            (a.first() >= b.first() && a.last() <= b.last())
                || (b.first() >= a.first() && b.last() <= a.last())
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .iter()
        .filter(|(a, b)| {
            (a.first() <= b.last() && a.last() >= b.first())
                || (b.first() <= a.last() && b.last() >= a.first())
        })
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
