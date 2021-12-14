use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use itertools::Itertools;

pub struct Cave {
    graph: Vec<Vec<usize>>,
    small_caves: BitFlags,
    end: usize,
}

type BitFlags = u64;

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
        .enumerate()
        .map(|(idx, n)| ((*n == n.to_lowercase()) as u64) << idx)
        .sum();

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

fn dfs(
    node: usize,
    cave: &Cave,
    mut seen: BitFlags,
    mut seen_twice: bool,
    memo: &mut HashMap<(bool, BitFlags, usize), usize>,
) -> usize {

    if node == cave.end {
        return 1;
    }

    let key = (seen_twice, seen, node);
    if let Some(paths) = memo.get(&key) {
        return *paths;
    }

    let mask = 1 << node;

    if cave.small_caves & mask != 0 {
        if seen & mask != 0 {
            seen_twice = true
        }
        seen |= mask;
    }

    let mut paths = 0;
    for &neighbor in cave.graph[node].iter() {
        if seen & (1 << neighbor) != 0 && seen_twice {
            continue;
        }

        paths += dfs(neighbor, cave, seen, seen_twice, memo)
    }

    memo.insert(key, paths);
    paths
}

#[aoc(day12, part1)]
pub fn part1(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    dfs(0, cave, 0, true, memo)
}

#[aoc(day12, part2)]
pub fn part2(cave: &Cave) -> usize {
    let memo = &mut HashMap::new();
    dfs(0, cave, 0, false, memo)
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
