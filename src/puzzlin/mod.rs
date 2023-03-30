use array2d::Array2D;
use heuristic::{eucl_heust, mann_heust, Heust};
pub mod heuristic;
pub mod solvers;
#[cfg(test)]
mod solverstest;
#[cfg(test)]
mod test;
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Puzzle {
    state: Array2D<u8>,
    pub neighbours: Vec<Puzzle>,
    pub parent: Option<Box<Puzzle>>,
    zeropos: (usize, usize),
    score: Array2D<u8>,
    cost: usize,
}

#[derive(Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Puzzle {
    pub fn getzero(self) -> (usize, usize) {
        self.zeropos
    }
    pub fn getstate(self) -> Array2D<u8> {
        self.state
    }
    pub fn getscore(self) -> Array2D<u8> {
        self.score
    }

    pub fn getcost(self) -> usize {
        self.cost
    }

    pub fn getmoves(self) -> Vec<Direction> {
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
    pub fn getchildren(mut self, h: Heust) -> Self {
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

    pub fn checkgoal(self) -> bool {
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
    pub fn equals(self, other: Puzzle) -> bool {
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

    pub fn calc_mann(mut self) -> Self {
        for index in self.state.indices_row_major() {
            if self.state[(index)] != 0 {
                // empty tile is not misplaced
                self.score[(index)] = mann_heust(index, self.state[(index)]) as u8;
            }
        }
        self
    }
    pub fn calc_eucl(mut self) -> Self {
        for index in self.state.indices_row_major() {
            if self.state[(index)] != 0 {
                self.score[(index)] = eucl_heust(index, self.state[(index)]) as u8;
            }
        }
        self
    }
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
