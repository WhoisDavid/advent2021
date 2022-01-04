use aoc_runner_derive::{aoc, aoc_generator};

type Position = u64;

#[aoc_generator(day21)]
pub fn input_parser(input: &str) -> (Position, Position) {
    let mut lines = input.lines();
    let p1 = lines.next().unwrap();
    let p1 = p1.rsplitn(2, ' ').next().unwrap().parse().unwrap();
    let p2 = lines.next().unwrap();
    let p2 = p2.rsplitn(2, ' ').next().unwrap().parse().unwrap();
    (p1, p2)
}

#[aoc(day21, part1)]
pub fn part1((mut p1, mut p2): &(Position, Position)) -> u64 {
    let mut score_p1 = 0;
    let mut score_p2 = 0;
    let mut die = 0;
    let mut die_roll = 0;
    p1 -= 1;
    p2 -= 1;
    while score_p1 < 1000 && score_p2 < 1000 {
        die_roll += 3;
        let roll = (die % 100) + (die + 1) % 100 + (die + 2) % 100 + 3;
        if die_roll % 2 == 1 {
            p1 = (p1 + roll) % 10;
            score_p1 += p1 + 1;
        } else {
            p2 = (p2 + roll) % 10;
            score_p2 += p2 + 1;
        }
        die = (die + 3) % 100;
    }

    die_roll * if score_p1 >= 1000 { score_p2 } else { score_p1 }
}

const ROLLS_FREQUENCY: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

// Counts the # of plays per # rounds
// Max # rounds is 10 (index 9) to get to 21
#[derive(Default)]
struct PlayCounter {
    winning: [u64; 10],
    losing: [u64; 10],
}

fn count_possible_plays(starting_pos: usize, possible_rolls: &[[usize; 7]; 10]) -> PlayCounter {
    let mut play_counter = PlayCounter::default();
    play(starting_pos, possible_rolls, &mut play_counter, 0, 0, 1);
    play_counter
}

fn play(
    pos: usize,
    possible_rolls: &[[usize; 7]; 10],
    play_counter: &mut PlayCounter,
    score: usize,
    round_count: usize,
    universes: u64,
) {
    for (next_pos, freq) in possible_rolls[pos - 1].iter().zip(ROLLS_FREQUENCY) {
        if score + next_pos < 21 {
            play_counter.losing[round_count] += universes * freq;
            play(
                *next_pos,
                possible_rolls,
                play_counter,
                score + next_pos,
                round_count + 1,
                universes * freq,
            );
        } else {
            play_counter.winning[round_count] += universes * freq;
        }
    }
}

#[aoc(day21, part2)]
pub fn part2((p1, p2): &(Position, Position)) -> u64 {
    let mut possible_rolls = [[0; 7]; 10];
    for (pos, rolls) in possible_rolls.iter_mut().enumerate() {
        for (roll, possible_roll) in rolls.iter_mut().enumerate() {
            *possible_roll = (pos + roll + 3) % 10 + 1;
        }
    }

    let p1_plays = count_possible_plays(*p1 as usize, &possible_rolls);
    let p2_plays = count_possible_plays(*p2 as usize, &possible_rolls);

    let p1_winning_universes = p1_plays
        .winning
        .iter()
        .skip(1) // if p1 wins, p2 last play is on the previous round
        .zip(p2_plays.losing)
        .map(|(p1_win, p2_lose)| p1_win * p2_lose)
        .sum::<u64>();
    let p2_winning_universes = p2_plays
        .winning
        .iter()
        .zip(p1_plays.losing)
        .map(|(p2_win, p1_lose)| p2_win * p1_lose)
        .sum::<u64>();

    std::cmp::max(p1_winning_universes, p2_winning_universes)
}

#[cfg(test)]
mod test_day21 {
    use super::*;

    const TESTCASE: &str = "\
Player 1 starting position: 4
Player 2 starting position: 8";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&input_parser(TESTCASE)), 739785)
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&input_parser(TESTCASE)), 444_356_092_776_315)
    }
}
