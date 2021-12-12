use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

type Cave = HashMap<String, Vec<String>>;

#[aoc_generator(day12)]
pub fn input_parser(input: &str) -> Cave {
    let mut cave = Cave::new();
    for path in input.lines() {
        let (a, b) = path.split_once('-').unwrap();
        if a != "end" && b != "start" {
            let edges = cave.entry(a.to_string()).or_default();
            edges.push(b.to_string());
        }
        if b != "end" && a != "start" {
            let edges = cave.entry(b.to_string()).or_default();
            edges.push(a.to_string());
        }
    }
    cave
}

fn memo_dfs_part1(
    node: &str,
    cave: &Cave,
    seen: HashSet<String>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = format!(
        "{}{}",
        seen.iter().cloned().sorted().collect::<String>(),
        node
    );

    if let Some(paths) = memo.get(&key) {
        *paths
    } else {
        let paths = dfs_part1(node, cave, seen, memo);
        memo.insert(key, paths);
        paths
    }
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

#[aoc(day12, part1)]
pub fn part1(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    memo_dfs_part1("start", cave, HashSet::with_capacity(10), memo)
}

fn memo_dfs_part2(
    node: &str,
    cave: &Cave,
    seen: HashSet<String>,
    seen_twice: bool,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = format!(
        "{}{}{}",
        seen_twice,
        seen.iter().cloned().sorted().collect::<String>(),
        node
    );

    if let Some(paths) = memo.get(&key) {
        *paths
    } else {
        let paths = dfs_part2(node, cave, seen, seen_twice, memo);
        memo.insert(key, paths);
        paths
    }
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
    for node in cave[node].iter() {
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
