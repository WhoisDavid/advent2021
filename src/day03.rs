use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Clone, Debug, Default)]
pub struct BitCounter {
    ones: u32,
    zeros: u32,
}

#[derive(Debug, Default)]
pub struct PowerConsumption {
    gamma: u32,
    epsilon: u32,
}

#[aoc(day3, part1)]
pub fn part1(input: &str) -> u32 {
    // Number of bits
    let nbits = input.lines().next().unwrap().len();
    let mut bit_counts = vec![BitCounter::default(); nbits];
    input.lines().for_each(|l| {
        l.char_indices().for_each(|(idx, char)| match char {
            '0' => bit_counts[idx].zeros += 1,
            '1' => bit_counts[idx].ones += 1,
            _ => unreachable!("Should only be ones and zeroes"),
        })
    });

    let rates = bit_counts
        .iter()
        .fold(PowerConsumption::default(), |acc, bc| PowerConsumption {
            gamma: (acc.gamma << 1) + (bc.ones > bc.zeros) as u32,
            epsilon: (acc.epsilon << 1) + (bc.ones < bc.zeros) as u32,
        });

    rates.gamma * rates.epsilon
}

pub enum LifeSupport {
    Oxygen,
    CO2,
}

fn life_support_rating(mut numbers: Vec<&str>, life_support: LifeSupport) -> u32 {
    let mut idx = 0;
    while numbers.len() > 1 {
        let mut bc = BitCounter::default();
        numbers.iter().for_each(|l| match l.chars().nth(idx) {
            Some('0') => bc.zeros += 1,
            Some('1') => bc.ones += 1,
            _ => unreachable!("Should only be ones and zeroes"),
        });

        let criteria = match life_support {
            LifeSupport::Oxygen => bc.ones >= bc.zeros,
            LifeSupport::CO2 => bc.ones < bc.zeros,
        };

        // Retain numbers matching the criteria
        numbers.retain(|l| l.chars().nth(idx) == Some(if criteria { '1' } else { '0' }));
        idx += 1;
    }
    // The remaining number is the rate
    u32::from_str_radix(numbers[0], 2).unwrap()
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> u32 {
    let lines = input.lines().collect_vec();
    let oxygen_rate = life_support_rating(lines.clone(), LifeSupport::Oxygen);
    let co2_rate = life_support_rating(lines, LifeSupport::CO2);
    oxygen_rate * co2_rate
}

#[cfg(test)]
mod test_ {
    use super::*;

    const TESTCASE: &str = "\
00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 198)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTCASE), 230)
    }
}
