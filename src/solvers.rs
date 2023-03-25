use std::collections::HashSet;

use crate::Puzzle;

pub struct Solution {
    goal_path: Vec<Puzzle>,
    cost: usize,
    explored: HashSet<Puzzle>,
}
impl Solution {
    fn from(&mut self, goal: Puzzle, explored: HashSet<Puzzle>) -> Self {
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
    fn new(goal: Puzzle, explored: HashSet<Puzzle>) -> Solution {
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
    pub fn get_path(self) -> Vec<Puzzle> {
        self.goal_path
    }
}
pub fn solve_dfs(mut frontier: Vec<Puzzle>, mut explored: HashSet<Puzzle>) -> Option<Solution> {
    match frontier.pop() {
        None => None,
        Some(node) => {
            explored.insert(node.clone());
            match node.clone().checkgoal() {
                true => Some(Solution::new(node.clone(), explored)),
                false => {
                    let children = node.clone().getchildren();
                    for child in children {
                        let mut dup = false;
                        for node in explored.clone() {
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        for node in frontier.clone() {
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            frontier.push(child)
                        }

                        println!(
                            "Frontier lenght: {} \n explored length: {} \n funny state: {:#?}",
                            frontier.clone().len(),
                            explored.clone().len(),
                            node.clone().state
                        );
                    }
                    solve_dfs(frontier, explored)
                }
            }
        }
    }
}
