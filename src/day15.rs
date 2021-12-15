use std::{
    cmp::{min, Reverse},
    collections::{BinaryHeap, VecDeque},
};

use aoc_runner_derive::{aoc, aoc_generator};

pub struct Input {
    graph: Vec<i32>,
    width: usize,
}
#[aoc_generator(day15)]
pub fn input_parser(input: &str) -> Input {
    let graph = input
        .lines()
        .flat_map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i32))
        .collect();
    let width = input.lines().next().unwrap().len();
    Input { graph, width }
}

fn dijkstra_scaled_heap(input: &Input, scale: usize) -> i32 {
    let map_width = scale * input.width;
    let map_len = scale * scale * input.graph.len();
    let mut dist = vec![i32::MAX; map_len];
    dist[0] = 0;

    let map_value = |node: usize| {
        let x = node / map_width;
        let y = node % map_width;
        let mut weight = input.graph[(x % input.width) * input.width + y % input.width];
        let shift = (x / input.width + y / input.width) as i32;
        weight += shift;
        if weight > 9 {
            weight - 9
        } else {
            weight
        }
    };

    let mut queue = BinaryHeap::from([(Reverse(0), 0)]);
    let mut seen = vec![false; map_len];
    seen[0] = true;
    while let Some((_, node)) = queue.pop() {
        if node == map_len - 1 {
            break;
        }

        [
            (node as i32 + 1, node % map_width != map_width - 1),
            (node as i32 - 1, node % map_width != 0),
            (node as i32 - map_width as i32, node >= map_width),
            (node as i32 + map_width as i32, node < map_len - map_width),
        ]
        .iter()
        .filter(|(_, condition)| *condition)
        .map(|(n, _)| *n as usize)
        .for_each(|neighbor| {
            if !seen[neighbor] {
                seen[neighbor] = true;
                dist[neighbor] = min(dist[neighbor], dist[node] + map_value(neighbor));
                queue.push((Reverse(dist[neighbor]), neighbor));
            }
        });
    }
    dist[dist.len() - 1]
}

fn dijkstra_scaled_queue(input: &Input, scale: usize) -> i32 {
    let map_width = scale * input.width;
    let map_len = scale * scale * input.graph.len();
    let mut dist = vec![i32::MAX; map_len];
    dist[0] = 0;

    let map_value = |node: usize| {
        let x = node / map_width;
        let y = node % map_width;
        let mut weight = input.graph[(x % input.width) * input.width + y % input.width];
        let shift = (x / input.width + y / input.width) as i32;
        weight += shift;
        if weight > 9 {
            weight - 9
        } else {
            weight
        }
    };

    let mut queue = VecDeque::with_capacity(map_len);
    queue.push_back(0);
    let mut seen = vec![false; map_len];
    seen[0] = true;
    while let Some(node) = queue.pop_front() {
        seen[node] = true;

        [
            (node as i32 + 1, node % map_width != map_width - 1),
            (node as i32 - 1, node % map_width != 0),
            (node as i32 - map_width as i32, node >= map_width),
            (node as i32 + map_width as i32, node < map_len - map_width),
        ]
        .iter()
        .filter(|(_, condition)| *condition)
        .map(|(n, _)| *n as usize)
        .for_each(|neighbor| {
            if dist[node] + map_value(neighbor) < dist[neighbor] {
                dist[neighbor] = min(dist[neighbor], dist[node] + map_value(neighbor));
                queue.push_back(neighbor);
            }
        });
    }
    dist[dist.len() - 1]
}

#[aoc(day15, part1)]
pub fn part1(input: &Input) -> i32 {
    dijkstra_scaled_heap(input, 1)
}

#[aoc(day15, part2, heap)]
pub fn part2(input: &Input) -> i32 {
    dijkstra_scaled_heap(input, 5)
}

#[aoc(day15, part2, queue)]
pub fn part2_queue(input: &Input) -> i32 {
    dijkstra_scaled_queue(input, 5)
}

#[cfg(test)]
mod test_day15 {
    use super::*;

    const TESTCASE: &str = "\
1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 40)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 315)
    }
}
