use std::fmt::Display;

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;

type Algorithm = [bool; 512];
type PixelMap = HashMap<(i32, i32), bool>;

#[derive(Clone, Debug)]
pub struct Image {
    pixels: PixelMap,
    buffer: PixelMap,
    xmin: i32,
    xmax: i32,
    ymin: i32,
    ymax: i32,
    infinite_pixel: bool,
}

impl Image {
    fn get_pixel(&self, x: i32, y: i32) -> bool {
        *self.pixels.get(&(x, y)).unwrap_or(&self.infinite_pixel)
    }

    fn set_pixel_in_buffer(&mut self, x: i32, y: i32, v: bool) {
        self.buffer.insert((x, y), v);
    }

    fn swap_buffer(&mut self) {
        std::mem::swap(&mut self.pixels, &mut self.buffer);
    }
}

impl Display for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.ymin..self.ymax {
            for x in self.xmin..self.xmax {
                write!(f, "{}", if self.get_pixel(x, y) { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day20)]
pub fn input_parser(input: &str) -> (Algorithm, Image) {
    let (algo, image) = input.split_once("\n\n").unwrap();
    let algo = algo
        .chars()
        .map(|c| c == '#')
        .collect::<Vec<_>>()
        .as_slice()
        .try_into()
        .unwrap();
    let pixels: PixelMap = image
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l.char_indices()
                .map(move |(x, c)| ((x as i32, y as i32), c == '#'))
        })
        .collect();

    let xmax = image.lines().next().unwrap().len() as i32;
    let ymax = image.lines().count() as i32;
    (
        algo,
        Image {
            pixels: pixels.clone(),
            buffer: pixels,
            xmin: 0,
            xmax,
            ymin: 0,
            ymax,
            infinite_pixel: false,
        },
    )
}

const CONVOLUTION: [(i32, i32); 9] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (0, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

fn convolution(image: &mut Image, algo: &Algorithm) {
    // Increase boundaries on each side
    image.xmin -= 1;
    image.xmax += 1;
    image.ymin -= 1;
    image.ymax += 1;

    // Apply convolution to each pixel in buffer
    for x in image.xmin..image.xmax {
        for y in image.ymin..image.ymax {
            let index = CONVOLUTION.iter().fold(0, |index, (dx, dy)| {
                (index << 1) + image.get_pixel(x + dx, y + dy) as usize
            });
            image.set_pixel_in_buffer(x, y, algo[index]);
        }
    }

    // Update infinite pixel
    image.infinite_pixel = if image.infinite_pixel {
        algo[algo.len() - 1]
    } else {
        algo[0]
    };

    image.swap_buffer()
}

#[aoc(day20, part1)]
pub fn part1((algo, image): &(Algorithm, Image)) -> usize {
    let mut image = image.clone();
    convolution(&mut image, algo);
    convolution(&mut image, algo);
    image.pixels.values().filter(|v| **v).count()
}

#[aoc(day20, part2)]
pub fn part2((algo, image): &(Algorithm, Image)) -> usize {
    let mut image = image.clone();
    (0..50).for_each(|_| convolution(&mut image, algo));
    // println!("{}", image);
    image.pixels.values().filter(|v| **v).count()
}

#[cfg(test)]
mod test_day20 {
    use super::*;

    const TESTCASE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 35)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 3351)
    }
}
