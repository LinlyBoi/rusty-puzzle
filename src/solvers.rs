use std::collections::{HashSet, VecDeque};

use crate::{find_index, init_puz, Heust, Puzzle};

#[derive(Debug, Clone)]
pub struct Solution {
    goal_path: Vec<Puzzle>,
    cost: usize,
    explored: HashSet<Puzzle>,
}
impl Solution {
    pub fn from_goal(
        goal: Puzzle,
        mut path: Vec<Puzzle>,
        explored: HashSet<Puzzle>,
        cost: usize,
    ) -> Solution {
        match goal.clone().parent {
            None => {
                let rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
                let actual_goal = init_puz(rows);
                path.push(actual_goal);
                Solution {
                    goal_path: path.clone(),
                    cost,
                    explored: explored.clone(),
                }
            }
            Some(puzzle) => {
                path.push(*puzzle.clone());
                Self::from_goal(*puzzle, path, explored, cost + 1)
            }
        }
    }
    pub fn get_path(self) -> Vec<Puzzle> {
        self.goal_path
    }
    pub fn get_cost(self) -> usize {
        self.cost
    }
}
pub fn solve_dfs(
    mut frontier: VecDeque<Puzzle>,
    mut explored: HashSet<Puzzle>,
) -> Option<Solution> {
    match frontier.pop_back() {
        None => None,
        Some(node) => {
            explored.insert(node.clone());
            match node.clone().checkgoal() {
                true => Some(Solution::from_goal(node.clone(), vec![], explored, 0)),
                false => {
                    let parent = node.clone().getchildren(Heust::NoH);
                    for child in parent.neighbours {
                        // if !checkvisited(child.clone(), false) {
                        //     frontier.push_back(child)
                        // }
                        //
                        let mut dup = false;
                        for node in explored.clone() {
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            frontier.push_back(child)
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
                true => Some(Solution::from_goal(node.clone(), vec![], explored, 0)),
                false => {
                    let parent = node.clone().getchildren(Heust::NoH);
                    for child in parent.neighbours {
                        let mut dup = false;
                        for node in explored.clone() {
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            frontier.push_back(child)
                        }
                    }
                    dbg!(
                        node.state,
                        frontier.clone().len(),
                        explored.clone().len(),
                        // explored.clone()
                    );
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

pub fn checkvisited(mut node: Puzzle, mut result: bool) -> bool {
    loop {
        match node.parent {
            None => break,
            Some(ref parent) => {
                if node.clone().equals(*parent.clone()) {
                    result = true;
                }
                match &parent.parent {
                    None => break,
                    Some(new_node) => node = *new_node.clone(),
                };
            }
        };
    }
    result
}
