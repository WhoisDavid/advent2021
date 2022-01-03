use std::hash::Hash;
use std::rc::Rc;

use aoc_runner_derive::aoc;
use hashbrown::HashMap;
use Node::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Node {
    Free = 4,
    A = 0,
    B = 1,
    C = 2,
    D = 3,
}

impl Node {
    fn energy(&self) -> usize {
        match self {
            Free => 0,
            A => 1,
            B => 10,
            C => 100,
            D => 1000,
        }
    }
}

const NUM_ROOMS: usize = 4;
const HALLWAY: [usize; 7] = [0, 1, 3, 5, 7, 9, 10];
const HALLWAY_LEN: usize = HALLWAY.len() + NUM_ROOMS;

type Board<const N: usize> = [Node; N];
type PossibleMoves = HashMap<(usize, usize), Vec<usize>>;

#[derive(Clone, PartialEq, Eq)]
pub struct Game<const N: usize, const R: usize> {
    board: Board<N>,
    moves: Rc<PossibleMoves>,
}

impl<const N: usize, const R: usize> Hash for Game<N, R> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

impl<const N: usize, const R: usize> Game<N, R> {
    // Generates the room indices
    const fn rooms() -> [[usize; R]; NUM_ROOMS] {
        let mut rooms = [[0; R]; NUM_ROOMS];
        let mut room = 0;
        while room < NUM_ROOMS {
            let mut idx = 0;
            while idx < R {
                rooms[room][idx] = HALLWAY_LEN + room + idx * NUM_ROOMS;
                idx += 1
            }
            room += 1
        }
        rooms
    }

    // Rooms indices
    const ROOMS: [[usize; R]; NUM_ROOMS] = Self::rooms();

    fn new(board: Board<N>) -> Self {
        Self {
            board,
            moves: Rc::new(Self::possible_moves()),
        }
    }

    fn possible_moves() -> PossibleMoves {
        let mut moves = HashMap::new();
        
        // Hallway <=> Room
        for loc in HALLWAY {
            for (i, room) in Self::ROOMS.iter().enumerate() {
                let entrance = i * 2 + 2;
                let mut path: Vec<usize> = if entrance > loc {
                    (loc + 1..=entrance).collect()
                } else {
                    (entrance..loc).rev().collect()
                };

                for r in room {
                    moves.insert((loc, *r), path.clone());
                    moves.insert((*r, loc), path.iter().rev().copied().collect());
                    path.push(*r);
                }
            }
        }

        // Room <=> Room
        for r1 in 0..Self::ROOMS.len() - 1 {
            for r2 in r1 + 1..Self::ROOMS.len() {
                let midpoint = 2 * r1 + 3;
                for rr1 in Self::ROOMS[r1] {
                    for rr2 in Self::ROOMS[r2] {
                        let mut path = moves.get(&(rr1, midpoint)).unwrap().clone();
                        let tmp = moves.get(&(midpoint, rr2)).unwrap().clone();
                        path.push(midpoint);
                        path.extend(tmp);
                        moves.insert((rr1, rr2), path.clone());
                        path.reverse();
                        moves.insert((rr2, rr1), path);
                    }
                }
            }
        }

        moves
    }

    fn in_final_position(&self, pod: usize) -> bool {
        let pod_type = self.board[pod];
        let room = Self::ROOMS.get(pod_type as usize).unwrap();

        for r in room.iter().rev() {
            if pod == *r {
                return true;
            } else if self.board[*r] != pod_type {
                return false;
            }
        }
        false
    }

    fn is_room_free(&self, pod: usize) -> Option<usize> {
        let pod_type = self.board[pod];
        let room = Self::ROOMS.get(pod_type as usize)?;

        let can_move = room
            .iter()
            .map(|r| self.board[*r])
            .skip_while(|n| *n == Free)
            .all(|n| n == pod_type);

        if can_move {
            room.iter().rfind(|r| self.board[**r] == Free).copied()
        } else {
            None
        }
    }

    fn get_path(&self, pod: usize, dst: usize) -> Option<&[usize]> {
        if self.board[dst] != Free {
            return None;
        }
        let path = self.moves.get(&(pod, dst))?;
        if path.iter().all(|i| self.board[*i] == Free) {
            Some(path)
        } else {
            None
        }
    }

    fn move_pod(&mut self, pod: usize, path: &[usize], dst: usize) -> Option<usize> {
        let cost = self.board[pod].energy() * (path.len() + 1);
        self.board.swap(pod, dst);
        Some(cost)
    }

    const fn final_pos() -> [Node; N] {
        let mut w = [Free; N];
        let mut i = HALLWAY_LEN;
        let pods = [A, B, C, D];
        while i < w.len() {
            w[i] = pods[(i - HALLWAY_LEN) % 4];
            i += 1;
        }
        w
    }

    const WIN: [Node; N] = Self::final_pos();

    fn done(&self) -> bool {
        self.board == Self::WIN
    }
}

impl<const N: usize, const R: usize> std::fmt::Debug for Game<N, R> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut nodes = self
            .board
            .iter()
            .map(|n| match n {
                Free => '.',
                A => 'A',
                B => 'B',
                C => 'C',
                D => 'D',
            })
            .enumerate()
            .fold(String::new(), |mut s, (i, c)| {
                s.push(c);
                if i >= HALLWAY_LEN {
                    s.push('#');
                }
                if i >= HALLWAY_LEN - 1 && (i - HALLWAY_LEN + 1) % NUM_ROOMS == 0 {
                    s.push('\n');
                    s.push_str(" #");
                }
                s
            });
        nodes.push_str("########");
        write!(f, "\n{}\n", nodes)
    }
}

fn parser<const N: usize, const R: usize>(input: &str) -> Game<N, R> {
    let board = input
        .lines()
        .flat_map(|s| {
            s.chars()
                .filter(|c| *c != '#' && *c != ' ')
                .map(|c| match c {
                    '.' => Free,
                    'A' => A,
                    'B' => B,
                    'C' => C,
                    'D' => D,
                    _ => unreachable!(),
                })
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    Game::new(board)
}

fn dfs<const N: usize, const R: usize>(
    game: Game<N, R>,
    cache: &mut HashMap<Game<N, R>, Option<usize>>,
) -> Option<usize> {
    if let Some(cost) = cache.get(&game) {
        return *cost;
    }

    if game.done() {
        return Some(0);
    }

    let min_cost = game
        .board
        .into_iter()
        .enumerate()
        .filter(|(pod, n)| *n != Free && !game.in_final_position(*pod))
        .map(|(pod, _)| pod)
        .filter_map(|pod| {
            if let Some(room) = game.is_room_free(pod) {
                let path = game.get_path(pod, room)?;
                let mut g = game.clone();
                Some(g.move_pod(pod, path, room)? + dfs(g, cache)?)
            } else {
                // move to hallway
                HALLWAY
                    .into_iter()
                    .filter_map(|loc| {
                        let path = game.get_path(pod, loc)?;
                        let mut g = game.clone();
                        Some(g.move_pod(pod, path, loc)? + dfs(g, cache)?)
                    })
                    .min()
            }
        })
        .min();

    cache.insert(game, min_cost);
    min_cost
}

#[aoc(day23, part1)]
pub fn part1(input: &str) -> Option<usize> {
    const ROOM_SIZE: usize = 2;
    const LEN: usize = HALLWAY.len() + NUM_ROOMS * (ROOM_SIZE + 1);
    let game: Game<LEN, ROOM_SIZE> = parser(&input);
    dfs(game, &mut HashMap::new())
}

const PART2: &str = "\
   #D#C#B#A#
   #D#B#A#C#";

#[aoc(day23, part2)]
pub fn part2(input: &str) -> Option<usize> {
    let pos = input.find("\n ")?;
    let mut input = input.to_string();
    input.insert_str(pos + 1, PART2);

    const ROOM_SIZE: usize = 4;
    const LEN: usize = HALLWAY.len() + NUM_ROOMS * (ROOM_SIZE + 1);
    let game: Game<LEN, ROOM_SIZE> = parser(&input);

    dfs(game, &mut HashMap::new())
}
