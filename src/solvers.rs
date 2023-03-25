use crate::Puzzle;

pub struct Solution {
    goal_path: Vec<Puzzle>,
    cost: usize,
    explored: Vec<Puzzle>,
}
impl Solution {
    fn from(&mut self, goal: Puzzle, explored: Vec<Puzzle>) -> Self {
        let mut path: Vec<Puzzle> = vec![];
        let mut cost: usize = 0;
        loop {
            match goal.clone().parent {
                None => {
                    self.goal_path = path.clone();
                    self.cost = cost;
                    self.explored = explored.clone();
                }
                Some(puzzle) => {
                    path.push(*puzzle);
                    cost += 1;
                }
            }
        }
    }
    fn new(goal: Puzzle, explored: Vec<Puzzle>) -> Solution {
        let mut path: Vec<Puzzle> = vec![];
        let mut cost: usize = 0;
        loop {
            match goal.clone().parent {
                None => Solution {
                    goal_path: path.clone(),
                    cost,
                    explored: explored.clone(),
                },
                Some(puzzle) => {
                    path.push(*puzzle);
                    cost += 1;
                    continue;
                }
            };
        }
    }
}
pub fn solve_dfs(puzzle: Puzzle, frontier: Vec<Puzzle>, explored: Vec<Puzzle>) -> Solution {
    // match frontier.pop() {
    //     None => Solution::new(puzzle, explored),
    //     Some(node) => {}
    // }
    todo!()
}
