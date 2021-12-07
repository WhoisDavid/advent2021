use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
pub fn input_parser(input: &str) -> Vec<i32> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

#[aoc(day7, part1, median)]
pub fn part1(input: &[i32]) -> i32 {
    let mut crabs = input.to_vec();
    crabs.sort();
    let median = crabs[crabs.len() / 2];
    crabs.iter().map(|c| (*c - median).abs()).sum()
}

#[aoc(day7, part2, dumb)]
pub fn part2(input: &[i32]) -> Option<i32> {
    let mut crabs = input.to_vec();
    crabs.sort();
    let start = *crabs.first().unwrap();
    let end = *crabs.last().unwrap();
    let mut min_fuel = None;

    for pos in start..=end {
        let fuel: i32 = crabs
            .iter()
            .map(|c| {
                let diff = (*c - pos).abs();
                diff * (diff + 1) / 2
            })
            .sum();
        min_fuel = match min_fuel {
            None => Some(fuel),
            Some(d) if d > fuel => Some(fuel),
            _ => break, // minimum is reached
        };
    }
    min_fuel
}

#[cfg(test)]
mod test_day7 {
    use super::*;

    const TESTCASE: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 37)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), Some(168))
    }
}
