use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

pub struct InputDumb {
    polymer: String,
    rules: HashMap<String, String>,
}

#[aoc_generator(day14, part1, dumb)]
pub fn input_parser_dumb(input: &str) -> InputDumb {
    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            (pair.to_string(), element.to_string())
        })
        .collect();

    InputDumb {
        polymer: polymer.to_string(),
        rules,
    }
}

/// len(n) = 2^n * (len(0)-1) + 1
fn step(p: &str, rules: &HashMap<String, String>) -> String {
    let mut new = String::from(&p[0..1]);
    for i in 0..p.len() - 1 {
        if let Some(ins) = rules.get(&p[i..i + 2]) {
            new.push_str(&format!("{}{}", ins, &p[i + 1..i + 2]))
        }
    }
    new
}

#[aoc(day14, part1, dumb)]
pub fn part1_dumb(input: &InputDumb) -> usize {
    let polymer = (0..10).fold(input.polymer.clone(), |s, _| step(&s, &input.rules));
    let element_count = polymer.chars().counts();
    element_count.values().max().unwrap() - element_count.values().min().unwrap()
}

pub struct Input {
    polymer: String,
    rules: HashMap<String, (String, String)>,
}

/// ABC => 1,12
/// AA -> B => AA -> AB, BA => 0 -> 1, 10
/// AB -> C => AB -> AC, CB => 1 -> 2, 11
/// ..
#[aoc_generator(day14)]
pub fn input_parser(input: &str) -> Input {
    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            (
                pair.to_string(),
                (
                    format!("{}{}", &pair[0..1], element),
                    format!("{}{}", element, &pair[1..2]),
                ),
            )
        })
        .collect();

    Input {
        polymer: polymer.to_string(),
        rules,
    }
}


fn solve(input: &Input, steps: usize) -> u64 {
    let polymer = &input.polymer;

    // Count each pair in the polymer
    let mut pair_count: HashMap<String, u64> = input
        .rules
        .iter()
        .map(|(pair, _)| (pair.clone(), 0))
        .collect();
    let mut next_pair_count = pair_count.clone();

    for i in 0..polymer.len() - 1 {
        pair_count
            .get_mut(&polymer[i..i + 2])
            .map(|count| *count += 1);
    }

    for _ in 0..steps {
        // Calculate next count
        pair_count
            .iter_mut()
            .filter(|(_, count)| **count > 0)
            .for_each(|(pair, cur_count)| {
                if let Some((p1, p2)) = input.rules.get(pair) {
                    next_pair_count
                        .get_mut(p1)
                        .map(|count| *count += *cur_count);
                    next_pair_count
                        .get_mut(p2)
                        .map(|count| *count += *cur_count);
                    *cur_count = 0;
                }
            });

        // Swap counts
        std::mem::swap(&mut pair_count, &mut next_pair_count);
    }

    // Count each element as # of pairs starting with element
    let mut element_count: HashMap<char, u64> = HashMap::new();
    pair_count.into_iter().for_each(|(s, count)| {
        let el = s.chars().next().unwrap();
        *element_count.entry(el).or_default() += count;
    });

    // Add the last element to the count
    element_count
        .entry(polymer.chars().last().unwrap())
        .and_modify(|e| *e += 1);

    element_count.values().max().unwrap() - element_count.values().min().unwrap()
}

#[aoc(day14, part1)]
pub fn part1(input: &Input) -> u64 {
    solve(input, 10)
}

#[aoc(day14, part2)]
pub fn part2(input: &Input) -> u64 {
    solve(input, 40)
}


/// ABC => 1,12
/// AA -> B => AA -> AB, BA => 0 -> 1, 10
/// AB -> C => AB -> AC, CB => 1 -> 2, 11
/// ..
#[aoc_generator(day14, part2, v2)]
pub fn input_parser_2(input: &str) -> Input {
    let (polymer, rules) = input.split_once("\n\n").unwrap();
    let pairs = rules.lines().map(|l| {
        let (pair, _) = l.split_once(" -> ").unwrap();
        pair
    }).collect::<Vec<_>>();
    
    let rules = rules
        .lines()
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            (
                pair.to_string(),
                (
                    format!("{}{}", &pair[0..1], element),
                    format!("{}{}", element, &pair[1..2]),
                ),
            )
        })
        .collect();

    Input {
        polymer: polymer.to_string(),
        rules,
    }
}


fn solve(input: &Input, steps: usize) -> u64 {
    let polymer = &input.polymer;

    // Count each pair in the polymer
    let mut pair_count: HashMap<String, u64> = input
        .rules
        .iter()
        .map(|(pair, _)| (pair.clone(), 0))
        .collect();
    let mut next_pair_count = pair_count.clone();

    for i in 0..polymer.len() - 1 {
        pair_count
            .get_mut(&polymer[i..i + 2])
            .map(|count| *count += 1);
    }

    for _ in 0..steps {
        // Calculate next count
        pair_count
            .iter_mut()
            .filter(|(_, count)| **count > 0)
            .for_each(|(pair, cur_count)| {
                if let Some((p1, p2)) = input.rules.get(pair) {
                    next_pair_count
                        .get_mut(p1)
                        .map(|count| *count += *cur_count);
                    next_pair_count
                        .get_mut(p2)
                        .map(|count| *count += *cur_count);
                    *cur_count = 0;
                }
            });

        // Swap counts
        std::mem::swap(&mut pair_count, &mut next_pair_count);
    }

    // Count each element as # of pairs starting with element
    let mut element_count: HashMap<char, u64> = HashMap::new();
    pair_count.into_iter().for_each(|(s, count)| {
        let el = s.chars().next().unwrap();
        *element_count.entry(el).or_default() += count;
    });

    // Add the last element to the count
    element_count
        .entry(polymer.chars().last().unwrap())
        .and_modify(|e| *e += 1);

    element_count.values().max().unwrap() - element_count.values().min().unwrap()
}

#[cfg(test)]
mod test_day14 {
    use super::*;

    const TESTCASE: &str = "\
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser_dumb(TESTCASE)), 1588)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser_dumb(TESTCASE)), 2188189693529)
    }
}
