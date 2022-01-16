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

type Board = Vec<Node>;
type PossibleMoves = HashMap<(usize, usize), Vec<usize>>;
type Rooms = [Vec<usize>; NUM_ROOMS];

#[derive(Clone)]
pub struct State {
    board: Board,
    game: Rc<Game>,
}

impl Hash for State {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.board.hash(state);
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.board == other.board
    }
}

impl Eq for State {}

struct Game {
    moves: PossibleMoves,
    rooms: Rooms,
    winning_pos: Board,
}

impl Game {
    fn new(room_size: usize) -> Self {
        let rooms = Self::rooms(room_size);
        Self {
            moves: Self::possible_moves(&rooms),
            rooms,
            winning_pos: Self::winning_position(room_size),
        }
    }
    // Generates the room indices
    fn rooms(room_size: usize) -> Rooms {
        let mut rooms = Rooms::default();
        for (room_idx, room) in rooms.iter_mut().enumerate() {
            for depth in 0..room_size {
                room.push(HALLWAY_LEN + room_idx + depth * NUM_ROOMS);
            }
        }
        rooms
    }

    // Generate possible moves
    fn possible_moves(rooms: &Rooms) -> PossibleMoves {
        let mut moves = HashMap::new();

        // Hallway <=> Room
        for loc in HALLWAY {
            for (i, room) in rooms.iter().enumerate() {
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
        for r1 in 0..rooms.len() - 1 {
            for r2 in r1 + 1..rooms.len() {
                let midpoint = 2 * r1 + 3;
                for rr1 in rooms[r1].iter() {
                    for rr2 in rooms[r2].iter() {
                        let mut path = moves.get(&(*rr1, midpoint)).unwrap().clone();
                        let tmp = moves.get(&(midpoint, *rr2)).unwrap().clone();
                        path.push(midpoint);
                        path.extend(tmp);
                        moves.insert((*rr1, *rr2), path.clone());
                        path.reverse();
                        moves.insert((*rr2, *rr1), path);
                    }
                }
            }
        }

        moves
    }

    // Generate the winning possition
    fn winning_position(room_size: usize) -> Board {
        let mut w = vec![Free; HALLWAY_LEN];
        w.extend([A, B, C, D].repeat(room_size));
        w
    }
}

impl State {
    fn new(board: Board) -> Self {
        let board_len = board.len();
        let room_size = (board_len - HALLWAY_LEN) / NUM_ROOMS;
        Self {
            board,
            game: Rc::new(Game::new(room_size)),
        }
    }

    fn in_final_position(&self, pod: usize) -> bool {
        let pod_type = self.board[pod];
        let room = self.game.rooms.get(pod_type as usize).unwrap();

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
        let room = self.game.rooms.get(pod_type as usize)?;

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
        let path = self.game.moves.get(&(pod, dst))?;
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

    fn done(&self) -> bool {
        self.board == self.game.winning_pos
    }
}

impl std::fmt::Debug for State {
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

fn parser(input: &str) -> State {
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
        .collect::<Vec<_>>();

    State::new(board)
}

fn dfs(game: State, cache: &mut HashMap<State, Option<usize>>) -> Option<usize> {
    if let Some(cost) = cache.get(&game) {
        return *cost;
    }

    if game.done() {
        return Some(0);
    }

    let min_cost = game
        .board
        .iter()
        .enumerate()
        .filter(|(pod, n)| **n != Free && !game.in_final_position(*pod))
        .map(|(pod, _)| pod)
        .filter_map(|pod| {
            // Move to room
            let min_cost = || -> Option<usize> {
                let room = game.is_room_free(pod)?;
                let path = game.get_path(pod, room)?;
                let mut g = game.clone();
                Some(g.move_pod(pod, path, room)? + dfs(g, cache)?)
            }();

            // If possible to move to room - do it
            if min_cost.is_some() {
                return min_cost;
            }

            // Move to hallway
            HALLWAY
                .into_iter()
                .filter_map(|loc| {
                    let path = game.get_path(pod, loc)?;
                    let mut g = game.clone();
                    Some(g.move_pod(pod, path, loc)? + dfs(g, cache)?)
                })
                .min()
        })
        .min();

    cache.insert(game, min_cost);
    min_cost
}

#[aoc(day23, part1, no_const)]
pub fn part1(input: &str) -> Option<usize> {
    let state: State = parser(input);
    dfs(state, &mut HashMap::new())
}

const PART2: &str = "\
   #D#C#B#A#
   #D#B#A#C#";

#[aoc(day23, part2, no_const)]
pub fn part2(input: &str) -> Option<usize> {
    let pos = input.find("\n ")?;
    let mut input = input.to_string();
    input.insert_str(pos + 1, PART2);

    let state: State = parser(&input);
    dfs(state, &mut HashMap::new())
}
