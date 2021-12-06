use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

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

#[aoc(day6, part1)]
pub fn part1(input: &[usize]) -> usize {
    simulate_lanternfish(input, 80)
}

#[aoc(day6, part2, array)]
pub fn part2(input: &[usize]) -> usize {
    simulate_lanternfish(input, 256)
}

#[aoc(day6, part2, golf_rotate)]
pub fn part2_golf(input: &[usize]) -> usize {
    const DAYS: usize = 256;
    let mut fish_counter = [0; NEW + 1];
    input.iter().for_each(|f| {
        fish_counter[*f] += 1;
    });

    for _ in 0..DAYS {
        fish_counter.rotate_left(1);
        // Fish at 0 reset to 6
        fish_counter[RESET] += fish_counter[NEW];
    }
    fish_counter.iter().sum()
}

#[aoc(day6, part2, fred_vecdeque)]
pub fn part2_fred_deque(input: &[usize]) -> usize {
    const DAYS: usize = 256;
    let mut fish_counter = VecDeque::from([0; NEW + 1]);
    input.iter().for_each(|f| {
        fish_counter[*f] += 1;
    });

    for _ in 0..DAYS {
        let new_gen = fish_counter.pop_front().unwrap();
        // Fish at 0 reset to 6
        fish_counter[RESET] += new_gen;
        // As many new fish are spawn
        fish_counter.push_back(new_gen);
    }
    fish_counter.iter().sum()
}

#[aoc(day6, part2, fred_array)]
pub fn part2_fred_array(input: &[usize]) -> usize {
    const DAYS: usize = 256;
    let mut deq = [0; 9 + DAYS];
    input.iter().for_each(|f| {
        deq[*f] += 1;
    });

    for d in 0..DAYS {
        let num_spawn = deq[d];
        deq[d + 7] += num_spawn;
        deq[d + 9] += num_spawn;
    }
    ((DAYS)..(DAYS + 9)).map(|x| deq[x]).sum()
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
