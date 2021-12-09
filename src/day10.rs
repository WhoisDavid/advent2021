use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<field>\w+)")]
pub struct Input {
    field: String,
}

#[aoc_generator(day10)]
pub fn input_parser(input: &str) -> Vec<Input> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
pub fn part1(_input: &[Input]) -> usize {
    unimplemented!()
}

#[aoc(day10, part2)]
pub fn part2(_input: &[Input]) -> usize {
    unimplemented!()
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    const TESTCASE: &str = "";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 0)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 0)
    }
}
