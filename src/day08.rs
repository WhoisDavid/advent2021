use aoc_runner_derive::{aoc, aoc_generator};
use hashbrown::HashSet;

/*
  0:      1:      2:      3:      4:
 aaaa            aaaa    aaaa
b    c       c       c       c  b    c
b    c       c       c       c  b    c
                 dddd    dddd    dddd
e    f       f  e            f       f
e    f       f  e            f       f
 gggg            gggg    gggg

  5:      6:      7:      8:      9:
 aaaa    aaaa    aaaa    aaaa    aaaa
b       b            c  b    c  b    c
b       b            c  b    c  b    c
 dddd    dddd            dddd    dddd
     f  e    f       f  e    f       f
     f  e    f       f  e    f       f
 gggg    gggg            gggg    gggg
*/

#[derive(Debug)]
pub struct Input {
    signals: Vec<HashSet<char>>,
    outputs: Vec<HashSet<char>>,
}

#[aoc_generator(day8)]
pub fn input_parser(input: &str) -> Vec<Input> {
    input
        .lines()
        .map(|s| {
            let (signal, output) = s.split_once(" | ").unwrap();
            let signal = signal
                .splitn(10, ' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            let output = output
                .splitn(4, ' ')
                .map(|s| s.chars().collect::<HashSet<_>>())
                .collect::<Vec<_>>();
            Input {
                signals: signal,
                outputs: output,
            }
        })
        .collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[Input]) -> usize {
    input
        .iter()
        .flat_map(|i| i.outputs.iter().filter(|s| [2, 3, 4, 7].contains(&s.len())))
        .count()
}

/// Work out the patterns to figure out which signal is which number:
/// First match unique length
/// - len 2 => 1
/// - len 3 => 7
/// - len 4 => 4
/// - len 7 => 8
/// Then intersect the sets:
/// - (len 5) & 1 == len 2 => 3
/// - (len 5) & 4 == len 2 => 2
/// - else (len 5) => 5
/// - (len 6) & 1 == len 1 => 6
/// - (len 6) & 4 == len 4 => 9
/// - else (len 6) => 0
pub fn match_digits(signals: &[HashSet<char>]) -> [usize; 10] {
    let mut map = [0; 10];

    // First map unique length
    for (idx, signal) in signals.iter().enumerate() {
        let matched_digit = match signal.len() {
            2 => 1,
            3 => 7,
            4 => 4,
            7 => 8,
            _ => continue, // not enough info
        };
        map[matched_digit] = idx;
    }

    // Then match based on # of matching segments
    for (idx, signal) in signals.iter().enumerate() {
        let intersect = |a: &HashSet<char>, digit| a.intersection(&signals[map[digit]]).count();

        let matched_digit = match signal.len() {
            5 if intersect(signal, 1) == 2 => 3,
            5 if intersect(signal, 4) == 2 => 2,
            5 => 5,
            6 if intersect(signal, 1) == 1 => 6,
            6 if intersect(signal, 4) == 4 => 9,
            6 => 0,
            _ => continue, // already mapped
        };

        map[matched_digit] = idx;
    }

    // Check all signals are allocated
    assert!((0..10).all(|d| map.contains(&d)));

    map
}

#[aoc(day8, part2)]
pub fn part2(inputs: &[Input]) -> usize {
    inputs
        .iter()
        .map(|input| {
            let digits = match_digits(&input.signals);
            input.outputs.iter().fold(0, |sum, output| {
                sum * 10
                    + digits
                        .iter()
                        .position(|d| &input.signals[*d] == output)
                        .expect("Match")
            })
        })
        .sum()
}

#[cfg(test)]
mod test_ {
    use super::*;

    const TESTCASE: &str = "\
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 26)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 61229)
    }
}
