use std::{ops::Add, str::FromStr};

use anyhow::anyhow;
use aoc_runner_derive::aoc;
use itertools::Itertools;
use Fish::*;

#[derive(Clone)]
pub enum Fish {
    Regular(u32),
    Tree(Box<Fish>, Box<Fish>),
}

impl Fish {
    fn get_regular(&self) -> Option<u32> {
        match self {
            Regular(v) => Some(*v),
            _ => None,
        }
    }
    fn propagate(&mut self, reg: u32, left_branch: bool) {
        match self {
            Regular(val) => *val += reg,
            Tree(left, _) if left_branch => left.propagate(reg, left_branch),
            Tree(_, right) if !left_branch => right.propagate(reg, left_branch),
            _ => unreachable!(),
        }
    }

    fn explode(&mut self, depth: u32) -> Option<(Option<u32>, Option<u32>)> {
        match self {
            Regular(_) => None,
            Tree(left, right) if depth == 4 => {
                // explosion
                let exploded = Some((left.get_regular(), right.get_regular()));
                *self = Fish::Regular(0);
                exploded
            }
            Tree(left, right) => {
                // propagate potential explosion values
                if let Some(mut explosion) = left.explode(depth + 1) {
                    // If explosion value to the right, propagate it
                    if let Some(r) = explosion.1.take() {
                        right.propagate(r, true)
                    };
                    Some(explosion)
                } else if let Some(mut explosion) = right.explode(depth + 1) {
                    // If explosion value to the left, propagate it
                    if let Some(r) = explosion.0.take() {
                        left.propagate(r, false)
                    };
                    Some(explosion)
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Regular(v) if *v > 9 => {
                *self = Fish::Tree(
                    Box::new(Fish::Regular(*v / 2)),
                    Box::new(Fish::Regular((*v + 1) / 2)),
                );
                true
            }
            Tree(left, right) => left.split() || right.split(),
            _ => false,
        }
    }

    fn reduce(&mut self) {
        while self.explode(0).is_some() || self.split() {}
    }

    fn magnitude(&self) -> u32 {
        match self {
            Regular(v) => *v,
            Tree(left, right) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }
}

impl FromStr for Fish {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('[') {
            Ok(Fish::Regular(s.parse()?))
        } else {
            // Pop first and last bracket
            let s = &s[1..s.len() - 1];
            let mut bracket = 0;
            let comma = 1 + s
                .chars()
                .position(|c| {
                    match c {
                        '[' => bracket += 1,
                        ']' => bracket -= 1,
                        _ => (),
                    };
                    bracket == 0
                })
                .ok_or(anyhow!("wrong grouping"))?;
            let (left, right) = (&s[0..comma], &s[comma + 1..]);
            Ok(Fish::Tree(
                Box::new(left.parse()?),
                Box::new(right.parse()?),
            ))
        }
    }
}

impl std::fmt::Debug for Fish {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fish::Regular(v) => write!(f, "{}", v),
            Fish::Tree(l, r) => write!(f, "[{:?},{:?}]", l, r),
        }
    }
}

impl Add<Fish> for Fish {
    type Output = Fish;

    fn add(self, rhs: Fish) -> Self::Output {
        let mut f = Fish::Tree(Box::new(self), Box::new(rhs));
        f.reduce();
        f
    }
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<Fish>().unwrap())
        .reduce(|a, b| a + b)
        .unwrap()
        .magnitude()
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|s| s.parse::<Fish>().unwrap())
        .permutations(2)
        .map(|mut p| (p.pop().unwrap() + p.pop().unwrap()).magnitude())
        .max()
        .unwrap()
}

#[cfg(test)]
mod test_day18 {
    use super::*;

    const TESTCASE: &str = "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 3488)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTCASE), 0)
    }
}
