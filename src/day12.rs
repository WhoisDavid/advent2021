use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

pub struct Cave {
    graph: Vec<Vec<usize>>,
    small_caves: Vec<bool>,
    end: usize,
}

#[aoc_generator(day12)]
pub fn input_parser(input: &str) -> Cave {
    let mut nodes = input
        .lines()
        .flat_map(|s| s.split('-'))
        .filter(|s| *s != "start" && *s != "end")
        .unique()
        .collect::<Vec<_>>();

    nodes.insert(0, "start");
    nodes.push("end");

    let start = 0;
    let end = nodes.len() - 1;

    let small_caves = nodes
        .iter()
        .map(|&n| n == n.to_lowercase())
        .collect::<Vec<_>>();

    let mut graph = vec![Vec::new(); nodes.len()];

    for path in input.lines() {
        let (a, b) = path.split_once('-').unwrap();
        let a = nodes.iter().position(|n| *n == a).unwrap();
        let b = nodes.iter().position(|n| *n == b).unwrap();

        if a != end && b != start {
            graph[a].push(b);
        }
        if b != end && a != start {
            graph[b].push(a);
        }
    }

    Cave {
        graph,
        small_caves,
        end,
    }
}

fn memo_dfs_part1(
    node: usize,
    cave: &Cave,
    seen: Vec<bool>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    let key = format!(
        "{}{}",
        seen.iter()
            .map(|b| if *b { '1' } else { '0' })
            .collect::<String>(),
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
    node: usize,
    cave: &Cave,
    mut seen: Vec<bool>,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if node == cave.end {
        return 1;
    }

    if cave.small_caves[node] {
        seen[node] = true;
    }

    let mut path = 0;
    for &nnode in cave.graph[node].iter() {
        if seen[nnode] {
            continue;
        }

        path += memo_dfs_part1(nnode, cave, seen.clone(), memo)
    }

    path
}

#[aoc(day12, part1)]
pub fn part1(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    memo_dfs_part1(0, cave, vec![false; cave.graph.len()], memo)
}

fn memo_dfs_part2(
    node: usize,
    cave: &Cave,
    seen: Vec<bool>,
    seen_twice: bool,
    memo: &mut HashMap<(bool, u32, usize), usize>,
) -> usize {
    let seen_int = seen.iter().fold(0, |s, b| (s + (*b as u32)) << 1);

    let key = (seen_twice, seen_int, node);
    if let Some(paths) = memo.get(&key) {
        *paths
    } else {
        let paths = dfs_part2(node, cave, seen, seen_twice, memo);
        memo.insert(key, paths);
        paths
    }
}

fn dfs_part2(
    node: usize,
    cave: &Cave,
    mut seen: Vec<bool>,
    mut seen_twice: bool,
    memo: &mut HashMap<(bool, u32, usize), usize>,
) -> usize {
    if node == cave.end {
        return 1;
    }

    if cave.small_caves[node] {
        if seen[node] {
            seen_twice = true
        }
        seen[node] = true
    }

    let mut path = 0;
    for &node in cave.graph[node].iter() {
        if seen[node] && seen_twice {
            continue;
        }
        path += memo_dfs_part2(node, cave, seen.clone(), seen_twice, memo)
    }

    path
}

#[aoc(day12, part2)]
pub fn part2(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    memo_dfs_part2(0, cave, vec![false; cave.graph.len()], false, memo)
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
