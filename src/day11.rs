use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

type Octopuses = HashMap<(i32, i32), i32>;

const OCTOPUSES_COUNT: usize = 100;

#[aoc_generator(day11)]
pub fn input_parser(input: &str) -> Octopuses {
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

const NEIGHBORS: [(i32, i32); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn octo_step(hm: &mut Octopuses, mut flashes: usize) -> usize {
    hm.values_mut().for_each(|e| *e += 1);
    let mut flashing = hm
        .iter()
        .filter(|(_, e)| **e > 9)
        .map(|(pos, _)| *pos)
        .collect::<VecDeque<_>>();

    let mut visited = [false; OCTOPUSES_COUNT];
    while let Some((x, y)) = flashing.pop_front() {
        if !visited[(x * 10 + y) as usize] {
            visited[(x * 10 + y) as usize] = true;
            // Count flash
            flashes += 1;
            // Reset octopus
            hm.insert((x, y), 0);
            // Update neighbors
            NEIGHBORS.iter().for_each(|(dx, dy)| {
                let neighbor = (x + dx, y + dy);
                if visited.get((neighbor.0 * 10 + neighbor.1) as usize) == Some(&false) {
                    if let Some(e) = hm.get_mut(&neighbor) {
                        *e += 1;
                        if *e > 9 {
                            flashing.push_back(neighbor)
                        }
                    }
                }
            })
        }
    }
    flashes
}

#[aoc(day11, part1)]
pub fn part1(hm: &Octopuses) -> usize {
    let mut hm = hm.clone();
    (0..100).fold(0, |flashes, _| octo_step(&mut hm, flashes))
}

#[aoc(day11, part2)]
pub fn part2(hm: &Octopuses) -> Option<usize> {
    let mut hm = hm.clone();
    (1..).find(|_| octo_step(&mut hm, 0) == OCTOPUSES_COUNT)
}

#[cfg(test)]
mod test_day11 {
    use super::*;

    const TESTCASE: &str = "\
5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 1656)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), Some(195))
    }
}
