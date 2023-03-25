use std::collections::HashSet;

use array2d::Array2D;

use crate::solvers::solve_dfs;

mod solvers;
#[cfg(test)]
mod solverstest;
#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
    // let rows = vec![vec![5, 2, 8], vec![4, 1, 7], vec![0, 3, 6]];
    let rows = vec![vec![1, 7, 2], vec![0, 4, 3], vec![8, 6, 5]];
    let mut test_puzzle = init_puz(rows, (0, 1));
    // _ = solve_dfs(vec![test_puzzle.clone()], HashSet::new()).expect("Nope");
    (test_puzzle.state, test_puzzle.zeropos) = test_puzzle.clone().move_zero(Direction::Up);
    println!("{:#?}", test_puzzle.state)
}
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Puzzle {
    state: Array2D<u8>,
    neighbours: Vec<Puzzle>,
    parent: Option<Box<Puzzle>>,
    zeropos: (usize, usize),
}
impl Puzzle {
    fn getchildren(self) -> Vec<Puzzle> {
        let mut children: Vec<Puzzle> = vec![];
        let moves = self.clone().getmoves();
        for direction in moves {
            let mut temp_child: Puzzle = self.clone();
            (temp_child.state, temp_child.zeropos) = self.clone().move_zero(direction);
            temp_child.parent = Some(Box::new(self.clone()));
            children.push(temp_child);
        }
        children
    }

    fn move_zero(self, dir: Direction) -> (Array2D<u8>, (usize, usize)) {
        let mut new_state = self.clone().state;
        let (x, y) = self.clone().getzero();
        let (zx, zy): (usize, usize);
        let temp_value: u8;

        match dir {
            Direction::Up => {
                temp_value = *new_state.get(x, y - 1).expect("No");
                (zx, zy) = (x, y - 1);
            }
            Direction::Down => {
                temp_value = *new_state.get(x, y + 1).expect("No");
                (zx, zy) = (x, y + 1);
            }
            Direction::Left => {
                temp_value = *new_state.get(x - 1, y).expect("No");
                (zx, zy) = (x - 1, y);
            }
            Direction::Right => {
                temp_value = *new_state.get(x + 1, y).expect("No");
                (zx, zy) = (x + 1, y);
            }
        }
        println!("{temp_value}, {}", new_state.get(1, 0).expect("A"));
        _ = new_state.set(y, x, temp_value);
        _ = new_state.set(zy, zx, 0);

        (new_state, (zx, zy))
    }

    fn getzero(self) -> (usize, usize) {
        self.zeropos
    }

    fn getmoves(self) -> Vec<Direction> {
        let mut moves: Vec<Direction> = vec![];
        match self.zeropos.0 {
            0 => moves.push(Direction::Right),
            1 => {
                moves.push(Direction::Right);
                moves.push(Direction::Left);
            }
            2 => moves.push(Direction::Left),
            _ => {}
        }
        match self.zeropos.1 {
            0 => moves.push(Direction::Down),
            1 => {
                moves.push(Direction::Down);
                moves.push(Direction::Up);
            }
            2 => moves.push(Direction::Up),
            _ => {}
        }
        moves
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

pub fn init_puz(rows: Vec<Vec<u8>>, (zx, zy): (usize, usize)) -> Puzzle {
    let test_state: Array2D<u8> = Array2D::from_rows(&rows).expect("no");
    Puzzle {
        neighbours: vec![],
        parent: None,
        state: test_state,
        zeropos: (zx, zy),
    }
}
