use std::fmt::Display;
use std::ops::Deref;
use std::sync::{Arc, RwLock};

use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashMap;
use rayon::ThreadPool;
use std::thread;

type Algorithm = [bool; 512];
type PixelMap = HashMap<(i32, i32), bool>;

#[derive(Clone, Debug)]
pub struct Image {
    pixels: PixelMap,
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

fn chunk_x_convolution(
    image: impl Deref<Target = Image>,
    algo: Arc<Algorithm>,
    chunk_size: usize,
    chunk_idx: usize,
) -> PixelMap {
    let chunk = (image.xmin..image.xmax)
        .skip(chunk_idx * chunk_size)
        .take(chunk_size);

    let mut buffer = PixelMap::with_capacity(chunk_size * (image.ymax - image.ymin + 1) as usize);

    // Apply convolution to each pixel in buffer
    for x in chunk {
        for y in image.ymin..image.ymax {
            let index = CONVOLUTION.iter().fold(0, |index, (dx, dy)| {
                (index << 1) + image.get_pixel(x + dx, y + dy) as usize
            });
            buffer.insert((x, y), algo[index]);
        }
    }

    buffer
}

fn convolution(image: &mut Image, algo: Arc<Algorithm>, threads: usize) {
    // Increase boundaries on each side
    image.xmin -= 1;
    image.xmax += 1;
    image.ymin -= 1;
    image.ymax += 1;

    let x_span = (image.xmax - image.xmin + 1) as usize;
    let chunk_size = x_span / threads + (x_span % threads > 0) as usize;

    let image_arc = Arc::new(image.clone());
    let mut handles = Vec::new();
    for chunk_idx in 0..threads {
        let image_arc = image_arc.clone();
        let algo = algo.clone();
        let handle = thread::spawn(move || {
            chunk_x_convolution(image_arc, algo, chunk_size as usize, chunk_idx as usize)
        });
        handles.push(handle);
    }

    for handle in handles {
        let chunk = handle.join().unwrap();
        image.pixels.extend(chunk);
    }

    // Update infinite pixel
    image.infinite_pixel = if image.infinite_pixel {
        algo[algo.len() - 1]
    } else {
        algo[0]
    };
}

fn convolution_rw(image_rw: Arc<RwLock<Image>>, algo: Arc<Algorithm>, threads: usize) {
    let (x_span, y_span) = {
        let mut image = image_rw.write().unwrap();
        // Increase boundaries on each side
        image.xmin -= 1;
        image.xmax += 1;
        image.ymin -= 1;
        image.ymax += 1;

        (
            (image.xmax - image.xmin + 1) as usize,
            (image.ymax - image.ymin + 1) as usize,
        )
    };

    let chunk_size = x_span / threads + (x_span % threads > 0) as usize;

    let mut handles = Vec::new();
    for chunk_idx in 0..threads {
        let image_arc = image_rw.clone();
        let algo = algo.clone();
        let handle = thread::spawn(move || {
            let image = image_arc.read().unwrap();
            chunk_x_convolution(image, algo, chunk_size as usize, chunk_idx as usize)
        });
        handles.push(handle);
    }

    let mut buffer = PixelMap::with_capacity(x_span * y_span);

    handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .for_each(|hm| buffer.extend(hm));

    let mut image = image_rw.write().unwrap();

    image.pixels = buffer;

    // Update infinite pixel
    image.infinite_pixel = if image.infinite_pixel {
        algo[algo.len() - 1]
    } else {
        algo[0]
    };
}

fn convolution_pooled_rw(
    image_rw: Arc<RwLock<Image>>,
    algo: Arc<Algorithm>,
    pool: &mut ThreadPool,
) {
    let (x_span, y_span) = {
        let mut image = image_rw.write().unwrap();
        // Increase boundaries on each side
        image.xmin -= 1;
        image.xmax += 1;
        image.ymin -= 1;
        image.ymax += 1;
        (
            (image.xmax - image.xmin + 1) as usize,
            (image.ymax - image.ymin + 1) as usize,
        )
    };

    let chunk_size = 10;
    let num_chunks = x_span / chunk_size + (x_span % chunk_size > 0) as usize;

    let (tx, rx) = std::sync::mpsc::channel();
    for chunk_idx in 0..num_chunks {
        let image = image_rw.clone();
        let algo = algo.clone();
        let tx = tx.clone();
        pool.spawn(move || {
            let image = image.read().unwrap();
            let buf = chunk_x_convolution(image, algo, chunk_size, chunk_idx);
            tx.send(buf).unwrap();
        });
    }

    drop(tx);

    let mut buffer = PixelMap::with_capacity(x_span * y_span);
    rx.into_iter().for_each(|hm| buffer.extend(hm));

    let mut image = image_rw.write().unwrap();
    image.pixels = buffer;

    // Update infinite pixel
    image.infinite_pixel = if image.infinite_pixel {
        algo[algo.len() - 1]
    } else {
        algo[0]
    };
}

#[aoc_generator(day20, part1, threads)]
pub fn parser_part1(input: &str) -> (Algorithm, Image) {
    input_parser(input)
}

#[aoc_generator(day20, part2, threads)]
pub fn parser_part2(input: &str) -> (Algorithm, Image) {
    input_parser(input)
}

const THREADS: usize = 5;

#[aoc(day20, part1, threadpool_rw)]
pub fn part1((algo, image): &(Algorithm, Image)) -> usize {
    let image = Arc::new(RwLock::new(image.clone()));
    let algo = Arc::new(*algo);
    let mut pool = rayon::ThreadPoolBuilder::new()
        .num_threads(THREADS)
        .build()
        .unwrap();

    (0..2).for_each(|_| {
        convolution_pooled_rw(image.clone(), algo.clone(), &mut pool);
    });

    let image = image.read().unwrap();
    image.pixels.values().filter(|v| **v).count()
}

const PART2: usize = 50;

#[aoc(day20, part2, threads)]
pub fn part2((algo, image): &(Algorithm, Image)) -> usize {
    let mut image = image.clone();
    let algo = Arc::new(*algo);

    (0..PART2).for_each(|_| convolution(&mut image, algo.clone(), THREADS));
    image.pixels.values().filter(|v| **v).count()
}

#[aoc(day20, part2, threads_rw)]
pub fn part2_rw((algo, image): &(Algorithm, Image)) -> usize {
    let image = Arc::new(RwLock::new(image.clone()));
    let algo = Arc::new(*algo);

    (0..PART2).for_each(|_| convolution_rw(image.clone(), algo.clone(), THREADS));

    let image = image.read().unwrap();
    image.pixels.values().filter(|v| **v).count()
}

#[aoc(day20, part2, threadpool_rw)]
pub fn part2_pooled_rw((algo, image): &(Algorithm, Image)) -> usize {
    let image = Arc::new(RwLock::new(image.clone()));
    let algo = Arc::new(*algo);

    let mut pool = rayon::ThreadPoolBuilder::new()
        .num_threads(THREADS)
        .build()
        .unwrap();

    (0..PART2).for_each(|_| convolution_pooled_rw(image.clone(), algo.clone(), &mut pool));

    let image = image.read().unwrap();
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
