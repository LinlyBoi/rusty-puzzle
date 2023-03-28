use crate::{find_index, Puzzle};

pub enum Heust {
    Mann,
    Eucl,
    NoH,
}
pub fn mann_heust((curr_x, curr_y): (usize, usize), value: u8, goal: Puzzle) -> usize {
    let (goal_x, goal_y) = find_index(goal.state, value).expect("not exist");
    curr_x.abs_diff(goal_x) + curr_y.abs_diff(goal_y)
}
pub fn eucl_heust((curr_x, curr_y): (usize, usize), value: u8, goal: Puzzle) -> usize {
    let (goal_x, goal_y) = find_index(goal.state, value).expect("not exist");
    let square = curr_x.pow(2).abs_diff(goal_x).pow(2) + curr_y.pow(2).abs_diff(goal_y).pow(2);
    (square as f64).sqrt() as usize
}
