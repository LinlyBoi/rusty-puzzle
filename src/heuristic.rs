use array2d::Array2D;

use crate::{find_index, Puzzle};

pub enum Heust {
    Mann,
    Eucl,
    NoH,
}
pub fn mann_heust((curr_x, curr_y): (usize, usize), value: u8) -> usize {
    let goal_rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let goal = Array2D::from_rows(&goal_rows).expect("failed to make 2D array");
    let (goal_x, goal_y) = find_index(goal, value).expect("not exist");
    curr_x.abs_diff(goal_x) + curr_y.abs_diff(goal_y)
}
pub fn eucl_heust((curr_x, curr_y): (usize, usize), value: u8) -> usize {
    let goal_rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let goal = Array2D::from_rows(&goal_rows).expect("failed to make 2D array");
    let (goal_x, goal_y) = find_index(goal, value).expect("not exist");
    let square = curr_x.pow(2).abs_diff(goal_x).pow(2) + curr_y.pow(2).abs_diff(goal_y).pow(2);
    (square as f64).sqrt() as usize
}
impl Puzzle {
    pub fn calc_mann(mut self) -> Self {
        for index in self.state.indices_row_major() {
            self.score[(index)] = mann_heust(index, self.state[(index)]) as u8;
        }
        self
    }
    pub fn calc_eucl(mut self) -> Self {
        for index in self.state.indices_row_major() {
            self.score[(index)] = eucl_heust(index, self.state[(index)]) as u8;
        }
        self
    }
}
