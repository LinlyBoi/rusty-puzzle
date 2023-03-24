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
        let child = self.clone();
        let (zx, zy) = child.clone().getzero();
        let state = self.state;
        child
    }

    fn move_zero(mut self, dir: Direction) -> Array2D<u8> {
        let mut new_state = self.clone().state;
        let (x, y) = self.clone().getzero();
        match dir {
            Direction::Up => {
                new_state[self.clone().getzero()] = new_state[(x, y - 1)];
                new_state[(x, y - 1)] = 0;
                self.zeropos = (x, y - 1);
                new_state
            }
            Direction::Down => {
                new_state[self.clone().getzero()] = new_state[(x, y + 1)];
                new_state[(x, y + 1)] = 0;
                self.zeropos = (x, y + 1);
                new_state
            }
            Direction::Left => {
                new_state[self.clone().getzero()] = new_state[(x - 1, y)];
                new_state[(x - 1, y)] = 0;
                self.zeropos = (x - 1, y);
                new_state
            }
            Direction::Right => {
                new_state[self.clone().getzero()] = new_state[(x + 1, y)];
                new_state[(x + 1, y)] = 0;
                self.zeropos = (x + 1, y);
                new_state
            }
        }
    }

    fn getzero(self) -> (usize, usize) {
        self.zeropos
    }
}
#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
