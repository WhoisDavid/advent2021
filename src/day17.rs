use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"target area: x=(?P<xmin>.+)\.\.(?P<xmax>.+), y=(?P<ymin>.+)\.\.(?P<ymax>.+)")]
pub struct Target {
    xmin: isize,
    xmax: isize,
    ymin: isize,
    ymax: isize,
}

#[aoc_generator(day17)]
pub fn input_parser(input: &str) -> Target {
    input.parse().unwrap()
}

fn simulate_probe(mut vx: isize, mut vy: isize, target: &Target) -> Option<isize> {
    let mut x = 0;
    let mut y = 0;
    let mut max_y = 0;
    let in_target =
        |x, y| (target.xmin..=target.xmax).contains(&x) && (target.ymin..=target.ymax).contains(&y);

    while !in_target(x, y) && x <= target.xmax && y >= target.ymin {
        x += vx;
        y += vy;

        vx -= vx.signum();
        vy -= 1;

        if y > max_y {
            max_y = y;
        }
    }

    if in_target(x, y) {
        Some(max_y)
    } else {
        None
    }
}

/// When hitting the x-axis, vy = -vy0 and the maximum possible speed at x=0 is (ymin + 1)
/// Therefore the max initial speed must be vy0 = -(ymin + 1).
/// The max height is 1 + 2 + ... + vy0 = vy0 * (vy0 + 1) / 2
/// So: max(y) = -(ymin + 1)*(1-(ymin + 1)) / 2 = ymin * (ymin+1) / 2
#[aoc(day17, part1, analytical)]
pub fn part1_analytical(input: &Target) -> isize {
    input.ymin * (input.ymin + 1) / 2
}

#[aoc(day17, part1, brute_force)]
pub fn part1_brute_force(input: &Target) -> isize {
    (0..=input.xmax)
        .cartesian_product(input.ymin..=-input.ymin)
        .filter_map(|(vx, vy)| simulate_probe(vx, vy, input))
        .max()
        .unwrap()
}

#[aoc(day17, part2)]
pub fn part2(input: &Target) -> usize {
    (0..=input.xmax)
        .cartesian_product(input.ymin..=-input.ymin)
        .filter_map(|(vx, vy)| simulate_probe(vx, vy, input))
        .count()
}
#[cfg(test)]
mod test_day17 {
    use super::*;

    const TESTCASE: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn test_part1() {
        assert_eq!(part1_analytical(&input_parser(TESTCASE)), 45)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 112)
    }
}
