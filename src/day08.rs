use aoc_runner_derive::{aoc, aoc_generator};
use itertools::FoldWhile::{Continue, Done};
use itertools::Itertools;
use ndarray::{self, Array2, Axis};

type Parsed = Array2<i32>;

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Parsed {
    let v: Vec<Vec<i32>> = input
        .lines()
        .map(|l| l.chars().map(|c| c.to_digit(10).unwrap() as i32).collect())
        .collect();
    Array2::from_shape_vec(
        (v.len(), v.first().unwrap().len()),
        v.iter().flatten().copied().collect(),
    )
    .unwrap()
}

fn find_visibles_from_outside(input: &Array2<i32>, visible: &mut Array2<bool>, axis: usize) {
    let f = |mut cur_max, (i, v): (&i32, &mut bool)| {
        if *i > cur_max {
            *v = true;
            cur_max = *i;
        }
        cur_max
    };

    let axis = Axis(axis);

    input
        .axis_iter(axis)
        .zip(visible.axis_iter_mut(axis))
        .for_each(|(inp, mut vis)| {
            inp.iter().zip(vis.iter_mut()).fold(-1, f);
            inp.iter().rev().zip(vis.iter_mut().rev()).fold(-1, f);
        });
}

#[aoc(day8, part1)]
fn part1(input: &Parsed) -> usize {
    let mut visible = Array2::<bool>::default(input.raw_dim());

    find_visibles_from_outside(input, &mut visible, 0);
    find_visibles_from_outside(input, &mut visible, 1);

    visible.iter().filter(|v| **v).count()
}

#[aoc(day8, part2)]
fn part2(input: &Parsed) -> u32 {
    let mut visible = Array2::<u32>::default(input.raw_dim());
    let sizes = input.raw_dim();

    for x in 0..sizes[1] {
        for y in 0..sizes[0] {
            let cur = *input.get((y, x)).unwrap();
            let f = |tot, tree: &i32| -> _ {
                if *tree >= cur {
                    Done(tot + 1)
                } else {
                    Continue(tot + 1)
                }
            };

            let row = input.row(y);
            let (left, right) = row.split_at(Axis(0), x);

            let can_see_left = left.iter().rev().fold_while(0, f).into_inner();
            let can_see_right = right.iter().skip(1).fold_while(0, f).into_inner();

            let col = input.column(x);
            let (up, down) = col.split_at(Axis(0), y);

            let can_see_up = up.iter().rev().fold_while(0, f).into_inner();
            let can_see_down = down.iter().skip(1).fold_while(0, f).into_inner();

            *visible.get_mut((y, x)).unwrap() =
                can_see_left * can_see_right * can_see_up * can_see_down;
        }
    }

    *visible.iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn sample1() {
        assert_eq!(part1(&parse_input(INPUT)), 21);
    }

    #[test]
    fn sample2() {
        assert_eq!(part2(&parse_input(INPUT)), 8);
    }
}
