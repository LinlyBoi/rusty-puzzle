use array2d::Array2D;
#[cfg(test)]
mod test;

fn main() {
    println!("Hello, world!");
}
#[derive(Debug, Clone)]
struct Puzzle {
    state: Array2D<u8>,
    neighbours: Vec<Puzzle>,
    parent: Option<Box<Puzzle>>,
    zeropos: (usize, usize),
}
impl Puzzle {
    fn get_child(self) -> Puzzle {
        todo!()
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
            2 => moves.push(Direction::Down),
            _ => {}
        }
        moves
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
