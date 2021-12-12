use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;
use recap::Recap;
use serde::Deserialize;

#[derive(Debug, Deserialize, Recap)]
#[recap(regex = r"(?P<field>\w+)")]
pub struct Input {
    field: String,
}

type Cave = HashMap<String, Vec<String>>;

#[aoc_generator(day12)]
pub fn input_parser(input: &str) -> Cave {
    let mut cave = Cave::new();
    for path in input.lines() {
        let (a, b) = path.split_once('-').unwrap();
        let edges = cave.entry(a.to_string()).or_default();
        edges.push(b.to_string());
        let edges = cave.entry(b.to_string()).or_default();
        edges.push(a.to_string());
    }
    cave
}

fn dfs_part1(
    node: &str,
    cave: &Cave,
    mut seen: HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == "end" {
        return 1;
    }

    if *node == node.to_lowercase() {
        seen.insert(node.to_string());
    }

    let mut path = 0;
    for node in cave[node].iter() {
        if seen.contains(node) {
            continue;
        }

        path += memo_dfs_part1(&node, cave, seen.clone(), memo)
    }

    path
}

fn memo_dfs_part1(
    node: &str,
    cave: &Cave,
    seen: HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if !memo.contains_key(node) {
        let paths = dfs_part1(node, cave, seen, memo);
        memo.insert(node.to_string(), paths);
    }
    memo[node]
}

#[aoc(day12, part1)]
pub fn part1(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    memo_dfs_part1("start", cave, HashSet::new(), memo)
}

fn memo_dfs_part2(
    node: &str,
    cave: &Cave,
    seen: HashSet<String>,
    seen_twice: bool,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let mut key: String = seen.iter().cloned().sorted().collect();
    if seen_twice {
        key = format!("{}{}{}", 2, key, node);
    } else {
        key = format!("{}{}", key, node);
    }
    if !memo.contains_key(&key) {
        let paths = dfs_part2(node, cave, seen, seen_twice, memo);
        memo.insert(key.clone(), paths);
    
    }
    memo[&key]
}

fn dfs_part2(
    node: &str,
    cave: &Cave,
    mut seen: HashSet<String>,
    mut seen_twice: bool,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == "end" {
        return 1;
    }

    if *node == node.to_lowercase() {
        if !seen.insert(node.to_string()) {
            seen_twice = true
        }
    }

    let mut path = 0;
    for node in cave[node].iter().filter(|n| *n != "start") {
        if seen.contains(node) && seen_twice {
            continue;
        }
        path += memo_dfs_part2(&node, cave, seen.clone(), seen_twice, memo)
    }

    path
}

#[aoc(day12, part2)]
pub fn part2(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    memo_dfs_part2("start", cave, HashSet::new(), false, memo)
}

#[cfg(test)]
mod test_day12 {
    use super::*;

    const TESTCASE: &str = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 226)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 3509)
    }
}
