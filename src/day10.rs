use aoc_runner_derive::aoc;

type Stack = Vec<char>;

fn match_parenthesis(s: &str) -> Result<Stack, char> {
    let mut stack = Stack::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '(' | '{' | '[' | '<' => stack.push(c),
            ')' | '}' | ']' | '>' => {
                let last_open = stack.pop().ok_or(c)?;
                match (last_open, c) {
                    ('(', ')') => (),
                    ('{', '}') => (),
                    ('<', '>') => (),
                    ('[', ']') => (),
                    _ => return Err(c),
                }
            }
            _ => unreachable!("No other chars than ()[]{}<>"),
        }
    }
    Ok(stack)
}

#[aoc(day10, part1)]
pub fn part1(input: &str) -> usize {
    input
        .lines()
        .filter_map(|s| match_parenthesis(s).err())
        .map(|c| match c {
            ')' => 3,
            ']' => 57,
            '}' => 1197,
            '>' => 25137,
            _ => panic!("Only closing brackets should be illegal"),
        })
        .sum()
}

#[aoc(day10, part2)]
pub fn part2(input: &str) -> usize {
    let mut scores = input
        .lines()
        .filter_map(|s| match_parenthesis(s).ok())
        .map(|stack| {
            stack.iter().rev().fold(0, |score, c| {
                score * 5
                    + match c {
                        '(' => 1,
                        '[' => 2,
                        '{' => 3,
                        '<' => 4,
                        _ => panic!("Only open brackets should be left on the stack!"),
                    }
            })
        })
        .collect::<Vec<_>>();
    scores.sort_unstable();
    scores[scores.len() / 2]
}

#[cfg(test)]
mod test_day10 {
    use super::*;

    const TESTCASE: &str = "
[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TESTCASE), 26397)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TESTCASE), 288957)
    }
}
