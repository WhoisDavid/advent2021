use aoc_runner_derive::{aoc, aoc_generator};
use Arg::*;
use Instruction::*;

type Register = usize;

#[derive(Debug)]
pub enum Instruction {
    Inp(Register),
    Add(Register, Arg),
    Mul(Register, Arg),
    Div(Register, Arg),
    Mod(Register, Arg),
    Eql(Register, Arg),
}

#[derive(Debug, PartialEq)]
pub enum Arg {
    Reg(Register),
    Lit(i32),
}

/// The program repeats a certain step 14 times with different variables (A, B, C here)
/// Step#   1    2   3    4    5    6    7    8    9   10   11   12   13   14
/// inp w
/// mul x   0    0   0    0    0    0    0    0    0    0    0    0    0    0
/// add x   z    z   z    z    z    z    z    z    z    z    z    z    z    z
/// mod x  26   26  26   26   26   26   26   26   26   26   26   26   26   26
/// div z   1    1   1   26    1   26   26    1   26    1    1   26   26   26 = A
/// add x  11   11  15  -14   10    0   -6   13   -3   13   15   -2   -9   -2 = B
/// eql x   w    w   w    w    w    w    w    w    w    w    w    w    w    w
/// eql x   0    0   0    0    0    0    0    0    0    0    0    0    0    0
/// mul y   0    0   0    0    0    0    0    0    0    0    0    0    0    0
/// add y  25   25  25   25   25   25   25   25   25   25   25   25   25   25
/// mul y   x    x   x    x    x    x    x    x    x    x    x    x    x    x
/// add y   1    1   1    1    1    1    1    1    1    1    1    1    1    1
/// mul z   y    y   y    y    y    y    y    y    y    y    y    y    y    y
/// mul y   0    0   0    0    0    0    0    0    0    0    0    0    0    0
/// add y   w    w   w    w    w    w    w    w    w    w    w    w    w    w
/// add y   6   14  13    1    6   13    6    3    8   14    4    7   15    1 = C
/// mul y   x    x   x    x    x    x    x    x    x    x    x    x    x    x
/// add z   y    y   y    y    y    y    y    y    y    y    y    y    y    y
///
/// Each step is a base 26 left/right shift
/// cond: w != z % 26 + B
/// z = if cond then { z/A * 26 + w + C } else { z / A }
/// A = 1 => B>= 10 => cond => z = z * 26 + w + C => left shift
/// => 7 steps with A = 1 and 7 with A = 26 => A = 26 needs to right shift (z = z / 26) meaning satisfy the cond
/// So A = 26 => w == z % 26 + B
#[aoc_generator(day24)]
pub fn input_parser(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|s| {
            let (op, reg, arg) = s
                .split_once(' ')
                .map(|(op, args)| {
                    let mut args = args.split_whitespace();
                    (op, args.next().unwrap(), args.next())
                })
                .unwrap();

            let reg = match reg {
                "w" => 0,
                "x" => 1,
                "y" => 2,
                "z" => 3,
                _ => panic!(),
            };

            if op == "inp" {
                Inp(reg)
            } else {
                let arg = match arg {
                    Some("w") => Reg(0),
                    Some("x") => Reg(1),
                    Some("y") => Reg(2),
                    Some("z") => Reg(3),
                    Some(d) => Lit(d.parse().unwrap()),
                    _ => unreachable!(),
                };
                match op {
                    "add" => Add(reg, arg),
                    "mul" => Mul(reg, arg),
                    "div" => Div(reg, arg),
                    "mod" => Mod(reg, arg),
                    "eql" => Eql(reg, arg),
                    _ => unreachable!(),
                }
            }
        })
        .collect()
}

/// Run the NOMAD instructions for a given input
fn nomad(input: [i32; 14], prog: &[Instruction]) -> i32 {
    // [w, x, y, z]
    let mut regs = [0, 0, 0, 0];

    macro_rules! match_arg {
        ($arg:ident) => {
            match $arg {
                Reg(reg) => regs[*reg],
                Lit(d) => *d,
            }
        };
    }

    let mut input_iter = input.iter();
    for op in prog {
        match op {
            Inp(r) => {
                regs[*r] = *input_iter.next().unwrap();
            }
            Add(r, arg) => regs[*r] += match_arg!(arg),
            Mul(r, arg) => regs[*r] *= match_arg!(arg),
            Div(r, arg) => regs[*r] /= match_arg!(arg),
            Mod(r, arg) => regs[*r] %= match_arg!(arg),
            Eql(r, arg) => regs[*r] = (regs[*r] == match_arg!(arg)) as i32,
        }
    }
    regs[3]
}

/// Solved in: https://docs.google.com/spreadsheets/d/1R-n3g3KqNJKzXcwrg4qJbLKGIOOBvQhmpG9DAlm6TAM
#[aoc(day24, part1)]
pub fn part1(prog: &[Instruction]) -> u64 {
    // Solved in spreadsheet
    let max_input = [5, 1, 9, 8, 3, 9, 9, 9, 9, 4, 7, 9, 9, 9];
    let z = nomad(max_input, prog);
    assert_eq!(z, 0);
    max_input.iter().fold(0, |s, d| s * 10 + *d as u64)
}

/// Solved in: https://docs.google.com/spreadsheets/d/1R-n3g3KqNJKzXcwrg4qJbLKGIOOBvQhmpG9DAlm6TAM
#[aoc(day24, part2)]
pub fn part2(prog: &[Instruction]) -> u64 {
    // Solved in spreadsheet
    let min_input = [1, 1, 2, 1, 1, 7, 9, 1, 1, 1, 1, 3, 6, 5];
    let z = nomad(min_input, prog);
    assert_eq!(z, 0);
    min_input.iter().fold(0, |s, d| s * 10 + *d as u64)
}