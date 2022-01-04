use std::env;
use std::io::{stdout, Write};
use std::{thread, time};

use aoc_runner_derive::{aoc, aoc_generator};
use termion::{color, cursor, screen};

use Node::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Node {
    Free,
    South,
    East,
}

#[derive(Clone)]
pub struct Map {
    map: Vec<Node>,
    buf1: Vec<Node>,
    buf2: Vec<Node>,
    width: usize,
}

impl Map {
    fn new(map: Vec<Node>, width: usize) -> Self {
        Self {
            map,
            buf1: Vec::new(),
            buf2: Vec::new(),
            width,
        }
    }

    fn south(&self, idx: usize) -> usize {
        (idx + self.width) % self.map.len()
    }

    fn east(&self, idx: usize) -> usize {
        if (idx + 1) % self.width == 0 {
            idx + 1 - self.width
        } else {
            idx + 1
        }
    }

    fn step(&mut self) -> bool {
        // Copy map in buf1
        self.buf1 = self.map.clone();

        // Move East in buf1
        for cur in 0..self.map.len() {
            let east = self.east(cur);
            if (self.map[cur], self.map[east]) == (East, Free) {
                self.buf1[cur] = Free;
                self.buf1[east] = East;
            }
        }

        // Copy buf1 in buf2
        self.buf2 = self.buf1.clone();

        // Move South in buf2
        for cur in 0..self.map.len() {
            let south = self.south(cur);
            if (self.buf1[cur], self.buf1[south]) == (South, Free) {
                self.buf2[cur] = Free;
                self.buf2[south] = South;
            }
        }

        // Swap Map and buf2
        std::mem::swap(&mut self.map, &mut self.buf2);

        self.map == self.buf2
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = String::new();
        self.map.iter().enumerate().for_each(|(i, n)| {
            let n = match n {
                Free => ' ',
                East => '>',
                South => 'v',
            };
            s.push(n);
            if (i + 1) % self.width == 0 {
                s.push('\n');
            }
        });
        write!(f, "{}", s)
    }
}

#[aoc_generator(day25)]
pub fn input_parser(input: &str) -> Map {
    let map = input
        .lines()
        .flat_map(|s| {
            s.chars().map(|c| match c {
                '.' => Free,
                '>' => East,
                'v' => South,
                _ => unreachable!(),
            })
        })
        .collect();
    let width = input.lines().next().unwrap().len();
    Map::new(map, width)
}

#[aoc(day25, part1)]
pub fn part1(map: &Map) -> usize {
    // Use env variable to toggle animation/color: e.g: `ANIMATION=1 COLOR=1 cargo aoc -d25`
    let animation = env::var("ANIMATION").is_ok();
    let color = env::var("COLOR").is_ok();

    let colorize = |map: &Map| {
        if color {
            map.to_string()
                .replace(">", &format!("{}>", color::Fg(color::Blue)))
                .replace("v", &format!("{}v", color::Fg(color::Red)))
        } else {
            map.to_string()
        }
    };

    let mut map = map.clone();

    // Altermate screen (with hidden cursor) for animation
    let mut screen = screen::AlternateScreen::from(cursor::HideCursor::from(stdout()));

    let final_step = (1..)
        .find(|step| {
            if animation {
                write!(
                    screen,
                    "{}{}Step {}:\n{}",
                    cursor::Goto(1, 1),
                    color::Fg(color::White),
                    step - 1,
                    colorize(&map)
                )
                .unwrap();
                screen.flush().unwrap();
                thread::sleep(time::Duration::from_millis(5));
            }
            map.step()
        })
        .unwrap();

    // Switch back to main screen
    write!(screen, "{}", screen::ToMainScreen).unwrap();

    if animation {
        // Print final map
        println!("Final step {}:\n{}", final_step, colorize(&map));
    }

    final_step
}

#[cfg(test)]
mod test_day25 {
    use super::*;

    const TESTCASE: &str = "\
v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 58)
    }
}
