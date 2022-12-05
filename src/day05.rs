use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use scan_fmt::scan_fmt;

#[derive(Debug)]
struct Move {
    n: usize,
    from: usize,
    to: usize,
}

type Parsed = (Vec<Vec<char>>, Vec<Move>);

#[aoc_generator(day5)]
fn parse_input(input: &str) -> Parsed {
    let mut split = input.split("\n\n");
    let drawing: Vec<&str> = split.next().unwrap().lines().collect();
    let moves = split.next().unwrap();

    let (idxs, towers) = drawing.split_last().unwrap();

    (
        idxs.chars()
            .enumerate()
            .filter(|(_, c)| !c.is_whitespace())
            .map(|(i, _)| {
                towers
                    .iter()
                    .map(|m| m.as_bytes()[i] as char)
                    .filter(|c| !c.is_whitespace())
                    .collect()
            })
            .collect(),
        moves
            .lines()
            .map(|l| {
                let (n, from, to) =
                    scan_fmt!(l, "move {} from {} to {}", usize, usize, usize).unwrap();
                Move { n, from, to }
            })
            .collect(),
    )
}

#[aoc(day5, part1)]
fn part1(input: &Parsed) -> String {
    let (towers, moves) = input;
    let mut towers = towers.clone();

    moves.iter().for_each(|m| {
        towers[m.from - 1]
            .drain(..m.n)
            .collect_vec()
            .iter()
            .for_each(|c| towers[m.to - 1].insert(0, *c));
    });

    towers.iter().map(|t| t[0]).join("")
}

#[aoc(day5, part2)]
fn part2(input: &Parsed) -> String {
    let (towers, moves) = input;
    let mut towers = towers.clone();

    moves.iter().for_each(|m| {
        towers[m.from - 1]
            .drain(..m.n)
            .collect_vec()
            .iter()
            .rev()
            .for_each(|c| towers[m.to - 1].insert(0, *c));
    });

    towers.iter().map(|t| t[0]).join("")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), "CMZ");
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), "MCD");
    }
}
