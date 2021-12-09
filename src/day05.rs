use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<x1>\d+),(?P<y1>\d+) -> (?P<x2>\d+),(?P<y2>\d+)")]
pub struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn to_points(&self) -> Vec<(i32, i32)> {
        let mut points = vec![(self.x2, self.y2)];
        let inc_x = (self.x2 - self.x1).signum();
        let inc_y = (self.y2 - self.y1).signum();
        let mut x = self.x1;
        let mut y = self.y1;
        while x != self.x2 || y != self.y2 {
            points.push((x, y));
            x += inc_x;
            y += inc_y;
        }
        points
    }
}

#[aoc_generator(day5)]
pub fn input_parser(input: &str) -> Vec<Line> {
    input.lines().map(|s| s.parse().unwrap()).collect()
}

#[aoc(day5, part1)]
pub fn part1(lines: &[Line]) -> usize {
    lines
        .iter()
        .filter(|Line { x1, y1, x2, y2 }| x1 == x2 || y1 == y2)
        .flat_map(|l| l.to_points())
        .sorted()
        .dedup_with_count()
        .filter(|(count, _)| *count > 1)
        .count()
}

#[aoc(day5, part2)]
pub fn part2(lines: &[Line]) -> usize {
    lines
        .iter()
        .flat_map(|l| l.to_points())
        .sorted()
        .dedup_with_count()
        .filter(|(count, _)| *count > 1)
        .count()
}

#[cfg(test)]
mod test_day05 {
    use super::*;

    const TESTCASE: &str = "\
0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 5)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 12)
    }
}
