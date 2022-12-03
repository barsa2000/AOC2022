use aoc_runner_derive::{aoc, aoc_generator};

type Parsed = Vec<(LhsMove, RhsMove)>;

// Should have kept the single letters :^)
enum LhsMove {
    Rock,
    Paper,
    Scissors,
}
enum RhsMove {
    RockOrLose,
    PaperOrDraw,
    ScissorsOrWin,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Parsed {
    input
        .lines()
        .map(|l| {
            let (lhs, rhs) = l.split_once(' ').unwrap();
            let lhs = match lhs {
                "A" => LhsMove::Rock,
                "B" => LhsMove::Paper,
                "C" => LhsMove::Scissors,
                _ => unreachable!(),
            };
            let rhs = match rhs {
                "X" => RhsMove::RockOrLose,
                "Y" => RhsMove::PaperOrDraw,
                "Z" => RhsMove::ScissorsOrWin,
                _ => unreachable!(),
            };
            (lhs, rhs)
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(input: &Parsed) -> u128 {
    input
        .iter()
        .map(|(lhs, rhs)| {
            let my_score = match rhs {
                RhsMove::RockOrLose => 1,
                RhsMove::PaperOrDraw => 2,
                RhsMove::ScissorsOrWin => 3,
                // _ => unreachable!()
            };
            let match_score = match (lhs, rhs) {
                (LhsMove::Rock, RhsMove::ScissorsOrWin) => 0, // lost
                (LhsMove::Rock, RhsMove::RockOrLose) => 3,    // draw
                (LhsMove::Rock, RhsMove::PaperOrDraw) => 6,   // win

                (LhsMove::Paper, RhsMove::PaperOrDraw) => 3, // draw
                (LhsMove::Paper, RhsMove::RockOrLose) => 0,  // lost
                (LhsMove::Paper, RhsMove::ScissorsOrWin) => 6, // win

                (LhsMove::Scissors, RhsMove::PaperOrDraw) => 0, // lost
                (LhsMove::Scissors, RhsMove::RockOrLose) => 6,  // win
                (LhsMove::Scissors, RhsMove::ScissorsOrWin) => 3, // draw
            };

            my_score + match_score
        })
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &Parsed) -> usize {
    input
        .iter()
        .map(|(lhs, rhs)| {
            let my_move = match (lhs, rhs) {
                (LhsMove::Rock, RhsMove::ScissorsOrWin) => RhsMove::PaperOrDraw, // win
                (LhsMove::Rock, RhsMove::PaperOrDraw) => RhsMove::RockOrLose,    // draw
                (LhsMove::Rock, RhsMove::RockOrLose) => RhsMove::ScissorsOrWin,  // lost

                (LhsMove::Paper, RhsMove::ScissorsOrWin) => RhsMove::ScissorsOrWin, // win
                (LhsMove::Paper, RhsMove::PaperOrDraw) => RhsMove::PaperOrDraw,     // draw
                (LhsMove::Paper, RhsMove::RockOrLose) => RhsMove::RockOrLose,       // lost

                (LhsMove::Scissors, RhsMove::ScissorsOrWin) => RhsMove::RockOrLose, // win
                (LhsMove::Scissors, RhsMove::PaperOrDraw) => RhsMove::ScissorsOrWin, // draw
                (LhsMove::Scissors, RhsMove::RockOrLose) => RhsMove::PaperOrDraw,   // lost
            };
            let my_score = match my_move {
                RhsMove::RockOrLose => 1,
                RhsMove::PaperOrDraw => 2,
                RhsMove::ScissorsOrWin => 3,
                // _ => unreachable!()
            };
            let match_score = match rhs {
                RhsMove::RockOrLose => 0,
                RhsMove::PaperOrDraw => 3,
                RhsMove::ScissorsOrWin => 6,
                // _ => unreachable!()
            };

            my_score + match_score
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn input<'a>() -> &'a str {
        "A Y
B X
C Z
"
    }

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(input())), 15);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(input())), 12);
    }
}
