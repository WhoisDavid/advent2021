use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::{HashMap, HashSet};
#[derive(Debug)]
pub struct Input {
    draw: Vec<u32>,
    boards: Vec<Vec<u32>>,
}

const WIDTH: u32 = 5;

#[aoc_generator(day4)]
pub fn input_parser(input: &str) -> Input {
    let (draw, boards) = input.split_once("\n\n").unwrap();
    let draw: Vec<u32> = draw.split(',').map(|d| d.parse().expect("int")).collect();
    let boards: Vec<Vec<u32>> = boards
        .split("\n\n")
        .map(|b| {
            b.split_whitespace()
                .map(|d| d.parse().expect("int"))
                .collect::<Vec<u32>>()
        })
        .collect();
    Input { draw, boards }
}

#[derive(Debug, Default, Clone)]
struct Bingo {
    row_counter: HashMap<usize, u32>,
    col_counter: HashMap<usize, u32>,
    drawn: HashSet<u32>,
}

impl Bingo {
    fn inc_row(&mut self, row_idx: usize) -> bool {
        let row = self.row_counter.entry(row_idx).or_insert(0);
        *row += 1;
        *row == WIDTH
    }

    fn inc_col(&mut self, col_idx: usize) -> bool {
        let col = self.col_counter.entry(col_idx).or_insert(0);
        *col += 1;
        *col == WIDTH
    }

    fn insert_and_check(&mut self, idx: usize, num: u32) -> bool {
        self.drawn.insert(num);
        let row_idx = idx / WIDTH as usize;
        let col_idx = idx % WIDTH as usize;
        let row_win = self.inc_row(row_idx);
        let col_win = self.inc_col(col_idx);
        row_win || col_win
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &Input) -> Option<u32> {
    let mut h = vec![Bingo::default(); input.boards.len()];
    for d in input.draw.iter() {
        for (board_idx, board_numbers) in input.boards.iter().enumerate() {
            for (idx, val) in board_numbers.iter().enumerate() {
                if d == val {
                    let board = &mut h[board_idx];
                    if board.insert_and_check(idx, *val) {
                        let winning_number = *d;
                        let unmarked_sum = board_numbers
                            .iter()
                            .filter(|num| !board.drawn.contains(num))
                            .sum::<u32>();
                        return Some(winning_number * unmarked_sum);
                    }
                }
            }
        }
    }
    None
}

#[aoc(day4, part2)]
pub fn part2(input: &Input) -> Option<u32> {
    let num_boards = input.boards.len();
    let mut h = vec![Bingo::default(); num_boards];
    let mut winning_boards = HashSet::new();
    for d in input.draw.iter() {
        for (board_idx, board_numbers) in input.boards.iter().enumerate() {
            // Skip winning boards
            if winning_boards.contains(&board_idx) {
                continue;
            }
            for (idx, val) in board_numbers.iter().enumerate() {
                if d == val {
                    let board = &mut h[board_idx];
                    if board.insert_and_check(idx, *val) {
                        winning_boards.insert(board_idx);
                        // Last one!
                        if winning_boards.len() == num_boards {
                            let winning_number = *d;
                            let unmarked_sum = board_numbers
                                .iter()
                                .filter(|num| !board.drawn.contains(num))
                                .sum::<u32>();
                            return Some(winning_number * unmarked_sum);
                        }
                    }
                }
            }
        }
    }
    None
}


#[cfg(test)]
mod test_day04 {
    use super::*;

    const TESTCASE: &str = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
    8  2 23  4 24
21  9 14 16  7
    6 10  3 18  5
    1 12 20 15 19

    3 15  0  2 22
    9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
    2  0 12  3  7";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), Some(4512))
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), Some(1924))
    }
}
