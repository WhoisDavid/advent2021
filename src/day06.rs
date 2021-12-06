use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::VecDeque;

#[aoc_generator(day6)]
pub fn input_parser(input: &str) -> Vec<usize> {
    input.split(',').map(|s| s.parse().unwrap()).collect()
}

const RESET: usize = 6;
const NEW: usize = 8;

fn simulate_lanternfish(fish: &[usize], days: usize) -> usize {
    let mut fish_counter = [0; NEW + 1];
    fish.iter().for_each(|f| {
        fish_counter[*f] += 1;
    });

    for _ in 0..days {
        // Day 0
        let new_gen = fish_counter[0];

        for d in 1..NEW + 1 {
            fish_counter[d - 1] = fish_counter[d]
        }

        // Fish at 0 reset to 6
        fish_counter[RESET] += new_gen;
        // As many new fish are spawn
        fish_counter[NEW] = new_gen;
    }

    fish_counter.iter().sum()
}

fn _simulate_lanternfish_fred(fish: &[usize], days: usize) -> usize {
    let mut fish_counter = VecDeque::from([0; NEW + 1]);
    fish.iter().for_each(|f| {
        fish_counter[*f] += 1;
    });

    for _ in 0..days {
        // Day 0 fish
        let new_gen = fish_counter.pop_front().unwrap();
        // Fish at 0 reset to 6
        fish_counter[RESET] += new_gen;
        // As many new fish are spawn
        fish_counter.push_back(new_gen);
    }

    fish_counter.iter().sum()
}

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
    simulate_lanternfish(input, 80)
}

#[aoc(day6, part2)]
pub fn part2(input: &[usize]) -> usize {
    simulate_lanternfish(input, 256)
}

#[cfg(test)]
mod test_ {
    use super::*;

    const TESTCASE: &str = "3,4,3,1,2";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 5934)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 2_6984_457_539)
    }
}
