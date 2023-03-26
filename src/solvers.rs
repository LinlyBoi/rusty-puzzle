use std::collections::{HashSet, VecDeque};

use crate::{find_index, Puzzle};

#[derive(Debug)]
pub struct Solution {
    goal_path: Vec<Puzzle>,
    cost: usize,
    explored: HashSet<Puzzle>,
}
impl Solution {
    pub fn from_goal(goal: Puzzle, explored: HashSet<Puzzle>, cost: usize) -> Solution {
        let mut path: Vec<Puzzle> = vec![];
        match goal.clone().parent {
            None => Solution {
                goal_path: path.clone(),
                cost,
                explored: explored.clone(),
            },
            Some(puzzle) => {
                path.push(*puzzle);
                Self::from_goal(goal, explored, cost + 1)
            }
        }
    }
    pub fn get_path(self) -> Vec<Puzzle> {
        self.goal_path
    }
}
pub fn solve_dfs(
    mut frontier: VecDeque<Puzzle>,
    mut explored: HashSet<Puzzle>,
) -> Option<Solution> {
    match frontier.pop_front() {
        None => None,
        Some(node) => {
            explored.insert(node.clone());
            match node.clone().checkgoal() {
                true => Some(Solution::from_goal(node.clone(), explored, 0)),
                false => {
                    let parent = node.clone().getchildren();
                    for child in parent.neighbours {
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
                            frontier.push_front(child)
                        }
                    }
                    dbg!(node.state, frontier.clone().len(), explored.clone().len());
                    solve_dfs(frontier, explored)
                }
            }
        }
    }
}
pub fn solve_bfs(
    mut frontier: VecDeque<Puzzle>,
    mut explored: HashSet<Puzzle>,
) -> Option<Solution> {
    match frontier.pop_front() {
        None => None,
        Some(node) => {
            explored.insert(node.clone());
            match node.clone().checkgoal() {
                true => Some(Solution::from_goal(node.clone(), explored, 0)),
                false => {
                    let children = node.clone().getchildren().neighbours;
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
                            frontier.push_front(node.clone());
                        }
                    }
                    dbg!(node.state, frontier.clone().len(), explored.clone().len());
                    solve_bfs(frontier, explored)
                }
            }
        }
    }
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
