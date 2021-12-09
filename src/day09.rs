use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
use std::collections::BinaryHeap;

type HeightMap = HashMap<(i32, i32), i32>;

#[aoc_generator(day9)]
pub fn input_parser(input: &str) -> HeightMap {
    input
        .lines()
        .enumerate()
        .flat_map(|(x, row)| {
            row.chars()
                .enumerate()
                .map(move |(y, height)| ((x as i32, y as i32), height.to_digit(10).unwrap() as i32))
        })
        .collect()
}

const NEIGHBORS: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

#[aoc(day9, part1)]
pub fn part1(hm: &HeightMap) -> i32 {
    hm.iter()
        .filter(|((x, y), height)| {
            NEIGHBORS
                .iter()
                .all(|(dx, dy)| hm.get(&(x + dx, y + dy)).map_or(true, |n| n > height))
        })
        .map(|(_, height)| height + 1)
        .sum()
}

fn traverse_basin(x: i32, y: i32, height: i32, hm: &HeightMap, basin: &mut HashSet<(i32, i32)>) {
    // Insert and check if already visited
    if basin.insert((x, y)) {
        NEIGHBORS
            .iter()
            .filter_map(|(dx, dy)| hm.get_key_value(&(x + dx, y + dy))) // get neighbors
            .filter(|(_, &nh)| nh > height && nh < 9) // filter for higher + < 9
            .for_each(|((nx, ny), nh)| traverse_basin(*nx, *ny, *nh, hm, basin))
    }
}

#[aoc(day9, part2)]
pub fn part2(hm: &HeightMap) -> usize {
    let low_points = hm.iter().filter(|((x, y), height)| {
        NEIGHBORS
            .iter()
            .all(|(dx, dy)| hm.get(&(x + dx, y + dy)).map_or(true, |n| n > height))
    });

    let basin = &mut HashSet::new();
    let basin_sizes = low_points
        .map(|((x, y), h)| {
            basin.clear();
            traverse_basin(*x, *y, *h, hm, basin);
            basin.len()
        })
        .collect::<BinaryHeap<_>>();

    basin_sizes.iter().take(3).product()
}



#[cfg(test)]
mod test_day09 {
    use super::*;

    const TESTCASE: &str = "\
2199943210
3987894921
9856789892
8767896789
9899965678";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 15)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 1134)
    }
}
