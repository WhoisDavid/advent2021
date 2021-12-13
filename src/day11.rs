use std::collections::VecDeque;

use aoc_runner_derive::{aoc, aoc_generator};

const OCTO_WIDTH: usize = 10;
const OCTOPUSES_COUNT: usize = OCTO_WIDTH * OCTO_WIDTH;
const RANGE: core::ops::Range<i32> = 0..OCTO_WIDTH as i32;

type Octopuses = [u32; OCTOPUSES_COUNT];

#[aoc_generator(day11)]
pub fn input_parser(input: &str) -> Octopuses {
    let mut octos = [0; OCTOPUSES_COUNT];
    input
        .chars()
        .filter(|c| *c != '\n')
        .enumerate()
        .for_each(|(idx, c)| octos[idx] = c.to_digit(10).unwrap());
    octos
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

fn octo_step(octopuses: &mut Octopuses, mut flashes: usize) -> usize {
    octopuses.iter_mut().for_each(|e| *e += 1);
    let mut flashing = octopuses
        .iter()
        .enumerate()
        .filter(|(_, e)| **e > 9)
        .map(|(pos, _)| pos)
        .collect::<VecDeque<_>>();

    let mut visited = [false; OCTOPUSES_COUNT];
    while let Some(pos) = flashing.pop_front() {
        if !visited[pos] {
            visited[pos] = true;
            // Count flash
            flashes += 1;
            // Reset octopus
            octopuses[pos] = 0;
            //
            let x = (pos / OCTO_WIDTH) as i32;
            let y = pos.rem_euclid(OCTO_WIDTH) as i32;
            // Update neighbors
            NEIGHBORS.iter().for_each(|(dx, dy)| {
                let (nx, ny) = (x + dx, y + dy);
                if !RANGE.contains(&nx) || !RANGE.contains(&ny) {
                    return;
                }

                let neighbor = (nx * OCTO_WIDTH as i32 + ny) as usize;
                if visited.get(neighbor) == Some(&false) {
                    if let Some(e) = octopuses.get_mut(neighbor) {
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
pub fn part1(octopuses: &Octopuses) -> usize {
    let mut octopuses = octopuses.clone();
    (0..100).fold(0, |flashes, _| octo_step(&mut octopuses, flashes))
}

#[aoc(day11, part2)]
pub fn part2(octopuses: &Octopuses) -> Option<usize> {
    let mut octopuses = octopuses.clone();
    (1..).find(|_| octo_step(&mut octopuses, 0) == OCTOPUSES_COUNT)
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
