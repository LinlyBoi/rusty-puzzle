use array2d::Array2D;
mod solvers;
#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone)]
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

        match dir {
            Direction::Up => {
                new_state[self.clone().getzero()] = new_state[(x, y - 1)];
                (zx, zy) = (x, y - 1);
            }
            Direction::Down => {
                new_state[self.clone().getzero()] = new_state[(x, y + 1)];
                (zx, zy) = (x, y + 1);
            }
            Direction::Left => {
                new_state[self.clone().getzero()] = new_state[(x - 1, y)];
                (zx, zy) = (x - 1, y);
            }
            Direction::Right => {
                new_state[self.clone().getzero()] = new_state[(x + 1, y)];
                (zx, zy) = (x + 1, y);
            }
        }
        new_state[(zx, zy)] = 0;

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
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
