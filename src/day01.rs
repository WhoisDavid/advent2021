use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
pub fn input_parser(input: &str) -> Vec<u32> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[u32]) -> usize {
    input.windows(2).filter(|w| w[1] > w[0]).count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[u32]) -> usize {
    input
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .tuple_windows::<(_, _)>()
        .filter(|w| w.1 > w.0)
        .count()
}
