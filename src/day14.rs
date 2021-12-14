use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

pub struct Input {
    polymer: String,
    rules: HashMap<String, (String, String)>,
}

#[aoc_generator(day14, string)]
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

pub struct Input2 {
    pairs: Vec<(char, char)>,
    pair_count: Vec<usize>,
    rules: Vec<(usize, usize)>,
    last_element: char,
}

/// ABC => 1,12
/// AA -> B => AA -> AB, BA => 0 -> 1, 10
/// AB -> C => AB -> AC, CB => 1 -> 2, 11
/// ..
#[aoc_generator(day14, v2)]
pub fn input_parser_2(input: &str) -> Input2 {
    let (polymer, rules) = input.split_once("\n\n").unwrap();

    let pairs = rules
        .lines()
        .map(|l| {
            let (pair, _) = l.split_once(" -> ").unwrap();
            let mut ch = pair.chars();
            (ch.next().unwrap(), ch.next().unwrap())
        })
        .collect::<Vec<_>>();

    let find_idx = |p1, p2| pairs.iter().position(|p| *p == (p1, p2)).unwrap();

    let mut rules = rules
        .lines()
        .map(|l| {
            let (pair, element) = l.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let (p1, p2) = (pair.next().unwrap(), pair.next().unwrap());
            let element = element.chars().next().unwrap();
            (
                find_idx(p1, p2),
                (find_idx(p1, element), find_idx(element, p2)),
            )
        })
        .collect::<Vec<_>>();
    rules.sort_by_key(|(idx, _)| *idx);
    let rules = rules.into_iter().map(|(_, v)| v).collect::<Vec<_>>();

    let mut pair_count = vec![0; pairs.len()];
    polymer
        .chars()
        .tuple_windows::<(_, _)>()
        .map(|(p1, p2)| find_idx(p1, p2))
        .for_each(|pair| pair_count[pair] += 1);

    let last_element = polymer.chars().last().unwrap();

    Input2 {
        pairs,
        pair_count,
        rules,
        last_element,
    }
}

fn solve_v2(input: &Input2, steps: usize) -> usize {
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

#[aoc(day14, part1, string)]
pub fn part1(input: &Input) -> u64 {
    solve(input, 10)
}

#[aoc(day14, part2, string)]
pub fn part2(input: &Input) -> u64 {
    solve(input, 40)
}

#[aoc(day14, part2, v2)]
pub fn part2_v2(input: &Input2) -> usize {
    solve_v2(input, 40)
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
        assert_eq!(part1(&input_parser(TESTCASE)), 1588)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 2188189693529);
        assert_eq!(part2_v2(&input_parser_2(TESTCASE)), 2188189693529);
    }
}
