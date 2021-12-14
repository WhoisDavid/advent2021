use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

pub struct InputVec {
    pairs: Vec<(char, char)>,
    pair_count: Vec<usize>,
    rules: Vec<(usize, usize)>,
    last_element: char,
}

/// ABC => 1,12
/// AA -> B => AA -> AB, BA => 0 -> 1, 10
/// AB -> C => AB -> AC, CB => 1 -> 2, 11
/// ..
#[aoc_generator(day14, vec_of_int)]
pub fn input_vec_parser(input: &str) -> InputVec {
    let (polymer, rules) = input.split_once("\n\n").unwrap();

    // Vec of pairs in rules (exhaustive)
    // e.g. [(A, B), (A, C), (C, B), (B, C), ..]
    let pairs = rules
        .lines()
        .map(|l| {
            let (pair, _) = l.split_once(" -> ").unwrap();
            let mut ch = pair.chars();
            (ch.next().unwrap(), ch.next().unwrap())
        })
        .collect::<Vec<_>>();

    // Map pair to index in `pairs`
    // e.g. (A, B) => 0, (A, C) => 1, ..
    let pairs_idx = pairs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(idx, pair)| (pair, idx))
        .collect::<HashMap<_, _>>();

    // Vec of rules (pair_idx => (pair1_idx, pair2_idx))
    // e.g. AB => C becomes AB => AC, CB which becomes 0 => (1, 2)
    let mut rules = rules
        .lines()
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let (p1, p2) = (pair.next().unwrap(), pair.next().unwrap());
            let element = element.chars().next().unwrap();
            (
                pairs_idx[&(p1, p2)],
                (pairs_idx[&(p1, element)], pairs_idx[&(element, p2)]),
            )
        })
        .collect::<Vec<_>>();

    // Sort by the index (same order as pair) and remove it
    rules.sort_by_key(|(idx, _)| *idx);
    let rules = rules.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    // Count pairs present in initial polymer
    // e.g. ABABC => AB: 2, BA: 1, BC: 1
    let mut pair_count = vec![0; pairs.len()];
    polymer
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|(p1, p2)| pairs_idx[&(p1, p2)])
        .for_each(|pair_idx| pair_count[pair_idx] += 1);

    // Save the last element of the polymer
    // e.g. ABABC => C
    let last_element = polymer.chars().last().unwrap();

    InputVec {
        pairs,
        pair_count,
        rules,
        last_element,
    }
}

fn solve_v2(input: &InputVec, steps: usize) -> usize {
    // Count each pair in the polymer
    let mut pair_count = input.pair_count.clone();
    let mut next_pair_count = vec![0; pair_count.len()];

    for _ in 0..steps {
        // Calculate next count
        pair_count
            .iter_mut()
            .enumerate()
            .filter(|(_, count)| **count > 0)
            .for_each(|(pair, cur_count)| {
                let (pair1, pair2) = input.rules[pair];
                next_pair_count[pair1] += *cur_count;
                next_pair_count[pair2] += *cur_count;
                *cur_count = 0;
            });

        // Swap counts
        std::mem::swap(&mut pair_count, &mut next_pair_count);
    }

    // Count each element as # of pairs starting with element
    let mut element_count: HashMap<char, usize> = HashMap::new();
    pair_count
        .into_iter()
        .enumerate()
        .for_each(|(pair, count)| {
            *element_count.entry(input.pairs[pair].0).or_default() += count;
        });

    // Add the last element to the count
    element_count
        .entry(input.last_element)
        .and_modify(|e| *e += 1);

    element_count.values().max().unwrap() - element_count.values().min().unwrap()
}

#[aoc(day14, part1, vec_of_int)]
pub fn part1(input: &InputVec) -> usize {
    solve_v2(input, 10)
}

#[aoc(day14, part2, vec_of_int)]
pub fn part2_vec_of_int(input: &InputVec) -> usize {
    solve_v2(input, 40)
}

/// Initial solution with HashMap (slower)
pub struct InputHashmap {
    polymer: String,
    rules: HashMap<String, (String, String)>,
}

#[aoc_generator(day14, part2, hashmap_of_string)]
pub fn input_hm_parser(input: &str) -> InputHashmap {
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

    InputHashmap {
        polymer: polymer.to_string(),
        rules,
    }
}

fn solve(input: &InputHashmap, steps: usize) -> u64 {
    let polymer = &input.polymer;

    // Count each pair in the polymer
    let mut pair_count: HashMap<String, u64> = input
        .rules
        .iter()
        .map(|(pair, _)| (pair.clone(), 0))
        .collect();
    let mut next_pair_count = pair_count.clone();

    for i in 0..polymer.len() - 1 {
        *pair_count.get_mut(&polymer[i..i + 2]).unwrap() += 1;
    }

    for _ in 0..steps {
        // Calculate next count
        pair_count
            .iter_mut()
            .filter(|(_, count)| **count > 0)
            .for_each(|(pair, cur_count)| {
                if let Some((p1, p2)) = input.rules.get(pair) {
                    *next_pair_count.get_mut(p1).unwrap() += *cur_count;
                    *next_pair_count.get_mut(p2).unwrap() += *cur_count;
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

#[aoc(day14, part2, hashmap_of_string)]
pub fn part2_hashmap_of_string(input: &InputHashmap) -> u64 {
    solve(input, 40)
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
        assert_eq!(part1(&input_vec_parser(TESTCASE)), 1588)
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2_hashmap_of_string(&input_hm_parser(TESTCASE)),
            2188189693529
        );
        assert_eq!(part2_vec_of_int(&input_vec_parser(TESTCASE)), 2188189693529);
    }
}
