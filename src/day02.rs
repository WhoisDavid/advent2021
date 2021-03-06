use aoc_runner_derive::{aoc, aoc_generator};
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Dir {
    Forward,
    Down,
    Up,
}

#[derive(Debug, PartialEq, Deserialize, Recap)]
#[recap(regex = r#"(?P<dir>\w+) (?P<dist>\d+)"#)]
pub struct Input {
    dir: Dir,
    dist: u32,
}

#[aoc_generator(day2)]
pub fn input_parser(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|s| s.parse().expect("Format: 'forward 50'"))
        .collect()
}

#[derive(Debug, Default)]
pub struct Pos {
    x: u32,
    y: u32,
    aim: u32,
}

#[aoc(day2, part1)]
pub fn part1(input: &[Input]) -> u32 {
    let pos = input.iter().fold(Pos::default(), |mut pos, inp| {
        match inp.dir {
            Dir::Forward => pos.x += inp.dist,
            Dir::Down => pos.y += inp.dist,
            Dir::Up => pos.y -= inp.dist,
        }
        pos
    });
    pos.x * pos.y
}

#[aoc(day2, part2)]
pub fn part2(input: &[Input]) -> u32 {
    let pos = input.iter().fold(Pos::default(), |mut pos, inp| {
        match inp.dir {
            Dir::Forward => {
                pos.x += inp.dist;
                pos.y += inp.dist * pos.aim
            }
            Dir::Down => pos.aim += inp.dist,
            Dir::Up => pos.aim -= inp.dist,
        }
        pos
    });
    pos.x * pos.y
}

#[cfg(test)]
mod test_day02 {
    use super::*;

    const TESTCASE: &str = "\
forward 5
down 5
forward 8
up 3
down 8
forward 2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 150)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 900)
    }
}
