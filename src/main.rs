use std::collections::HashSet;

use array2d::Array2D;
use heuristic::Heust;
use priority_queue::DoublePriorityQueue;

use crate::solvers::solve_aystar;

mod heuristic;
mod solvers;
#[cfg(test)]
mod solverstest;
#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
    // let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
    let rows = vec![vec![1, 4, 2], vec![3, 0, 8], vec![6, 5, 7]];
    // let test_puzzle = init_puz(rows);
    // let mut vec_q: VecDeque<Puzzle> = VecDeque::new();
    // vec_q.push_back(test_puzzle.clone());
    // let solly = solve_dfs(vec_q, HashSet::new()).expect("Nope");
    // let solly = solve_bfs(vec_q, HashSet::new()).expect("BFS is sucks");
    let test_puzzle = init_puz(rows).calc_mann();
    let init_h: usize = test_puzzle
        .score
        .elements_row_major_iter()
        .sum::<u8>()
        .into();
    let mut pq: DoublePriorityQueue<Puzzle, usize> = DoublePriorityQueue::new();
    pq.push(test_puzzle, init_h);
    let solly = solve_aystar(pq, HashSet::new(), crate::heuristic::Heust::Mann).expect("Nope");
    for step in solly.clone().get_path() {
        dbg!("funny path:", step.state);
    }
    dbg!(solly.clone().get_cost());
    for exp in solly.get_explored() {
        dbg!(exp.cost);
    }
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Puzzle {
    state: Array2D<u8>,
    neighbours: Vec<Puzzle>,
    parent: Option<Box<Puzzle>>,
    zeropos: (usize, usize),
    score: Array2D<u8>,
    cost: usize,
}
impl Puzzle {
    fn getzero(self) -> (usize, usize) {
        self.zeropos
    }

    fn getmoves(self) -> Vec<Direction> {
        let mut moves: Vec<Direction> = vec![];
        match self.zeropos.0 {
            0 => moves.push(Direction::Down),
            1 => {
                moves.push(Direction::Down);
                moves.push(Direction::Up);
            }
            2 => moves.push(Direction::Up),
            _ => {}
        }
        match self.zeropos.1 {
            0 => moves.push(Direction::Right),
            1 => {
                moves.push(Direction::Right);
                moves.push(Direction::Left);
            }
            2 => moves.push(Direction::Left),
            _ => {}
        }
        moves
    }
    fn move_zero(mut self, dir: Direction) -> Self {
        let old_state = self.clone().state;
        let (x, y) = self.clone().getzero();
        let (zx, zy): (usize, usize);
        let temp_value: u8;

        match dir {
            Direction::Up => {
                temp_value = old_state[(x - 1, y)];
                (zx, zy) = (x - 1, y);
            }
            Direction::Down => {
                temp_value = old_state[(x + 1, y)];
                (zx, zy) = (x + 1, y);
            }
            Direction::Left => {
                temp_value = old_state[(x, y - 1)];
                (zx, zy) = (x, y - 1);
            }
            Direction::Right => {
                temp_value = old_state[(x, y + 1)];
                (zx, zy) = (x, y + 1);
            }
        }
        self.state[(x, y)] = temp_value;
        self.state[(zx, zy)] = 0;
        self.zeropos = (zx, zy);
        self
    }
    fn getchildren(mut self, h: Heust) -> Self {
        let moves = self.clone().getmoves();
        let mut children: Vec<Puzzle> = vec![];
        for direction in moves {
            let mut temp_child: Puzzle;
            temp_child = self.clone().move_zero(direction);
            temp_child.parent = Some(Box::new(self.clone()));
            children.push(temp_child);
        }
        match h {
            Heust::NoH => {}
            Heust::Mann => {
                let mut mann_children: Vec<Puzzle> = vec![];
                for mut child in children.clone() {
                    child.cost = self.cost + 1;
                    mann_children.push(child.calc_mann());
                }
            }
            Heust::Eucl => {
                let mut eucl_children: Vec<Puzzle> = vec![];
                for mut child in children.clone() {
                    child.cost = self.cost + 1;
                    eucl_children.push(child.calc_eucl());
                }
            }
        }
        self.neighbours = children;
        self
    }

    fn checkgoal(self) -> bool {
        //hard coded for now
        let rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        let goal: Array2D<u8> = Array2D::from_rows(&rows).expect("no");
        let matching = goal
            .as_row_major()
            .iter()
            .zip(&self.state.as_row_major())
            .filter(|&(a, b)| a == b)
            .count();
        matching == goal.row_len() * goal.column_len()
    }
    fn equals(self, other: Puzzle) -> bool {
        let other_state = other.state;
        let matching = other_state
            .as_row_major()
            .iter()
            .zip(&self.state.as_row_major())
            .filter(|&(a, b)| a == b)
            .count();
        // dbg!(other_state.clone());
        // dbg!(self.clone());
        matching == other_state.row_len() * other_state.column_len()
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub fn init_puz(rows: Vec<Vec<u8>>) -> Puzzle {
    let test_state: Array2D<u8> = Array2D::from_rows(&rows).expect("no");
    let (zx, zy): (usize, usize) = find_index(test_state.clone(), 0).expect("OUT OF BOUNDS");
    Puzzle {
        neighbours: vec![],
        parent: None,
        state: test_state,
        zeropos: (zx, zy),
        score: Array2D::filled_with(0, 3, 3),
        cost: 0,
    }
}
pub fn find_index(twod: Array2D<u8>, value: u8) -> Option<(usize, usize)> {
    twod.indices_row_major()
        .find(|&index| twod[(index)] == value)
}
