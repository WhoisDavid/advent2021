use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::cmp::Ordering;
pub enum Fold {
    X(u32),
    Y(u32),
}

type Dots = Vec<(u32, u32)>;

pub struct Input {
    dots: Dots,
    folds: Vec<Fold>,
}

#[aoc_generator(day13)]
pub fn input_parser(input: &str) -> Input {
    let (coords, folds) = input.split_once("\n\n").unwrap();
    let dots = coords
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse().unwrap(), y.parse().unwrap())
        })
        .collect();
    let folds = folds
        .lines()
        .map(|s| {
            let (axis, val) = s["fold along ".len()..].split_once('=').unwrap();
            let val: u32 = val.parse().unwrap();
            match axis {
                "x" => Fold::X(val),
                "y" => Fold::Y(val),
                _ => panic!("Only x & y"),
            }
        })
        .collect();
    Input { dots, folds }
}

fn fold_paper(dots: Dots, fold: &Fold) -> Dots {
    match fold {
        Fold::X(folding_axis) => dots
            .into_iter()
            .filter_map(|(x, y)| match x.cmp(&folding_axis) {
                Ordering::Less => Some((x, y)),
                Ordering::Equal => None,
                Ordering::Greater => Some((2 * folding_axis - x, y)),
            })
            .unique() // remove overlapping points
            .collect(),
        Fold::Y(folding_axis) => dots
            .into_iter()
            .filter_map(|(x, y)| match y.cmp(&folding_axis) {
                Ordering::Less => Some((x, y)),
                Ordering::Equal => None,
                Ordering::Greater => Some((x, 2 * folding_axis - y)),
            })
            .unique() // remove overlapping points
            .collect(),
    }
}

#[aoc(day13, part1)]
pub fn part1(input: &Input) -> usize {
    fold_paper(input.dots.clone(), &input.folds[0]).len()
}

/// Coordinates are unusual
/// 0 -- x
/// |
/// y
fn print_dots(mut dots: Dots) {
    dots.sort();
    let xmax = *dots.iter().map(|(x, _)| x).max().unwrap();
    let ymax = *dots.iter().map(|(_, y)| y).max().unwrap();
    for line in 0..=ymax {
        let row = dots
            .iter()
            .filter(|(_, y)| *y == line)
            .map(|(x, _)| *x)
            .collect::<Vec<_>>();
        let row = (0..=xmax)
            .map(|x| if row.contains(&x) { '#' } else { ' ' })
            .collect::<String>();
        println!("{}", row);
    }
}

#[aoc(day13, part2)]
pub fn part2(input: &Input) -> &'static str {
    let dots = input
        .folds
        .iter()
        .fold(input.dots.clone(), |dots, fold| fold_paper(dots, fold));
    print_dots(dots);
    "Code is printed above!"
}

#[cfg(test)]
mod test_day13 {
    use super::*;

    const TESTCASE: &str = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 17)
    }
}
