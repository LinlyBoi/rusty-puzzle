use std::collections::{HashSet, VecDeque};

use priority_queue::DoublePriorityQueue;

use super::{Heust, Puzzle};

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
        match goal.parent {
            None => Solution {
                goal_path: path.clone(),
                cost,
                explored,
            },
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
    pub fn get_explored(self) -> HashSet<Puzzle> {
        self.explored
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
                true => Some(Solution::from_goal(node.clone(), vec![node], explored, 0)),
                false => {
                    let parent = node.getchildren(Heust::NoH);
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
                    // dbg!(node.state, frontier.clone().len(), explored.clone().len());
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
                true => Some(Solution::from_goal(node.clone(), vec![node], explored, 0)),
                false => {
                    let parent = node.getchildren(Heust::NoH);
                    for child in parent.neighbours {
                        let mut dup = false;
                        for node in explored.clone() {
                            //Iterate over explored to prevent
                            //duplicate states
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            frontier.push_back(child)
                        }
                    }
                    solve_bfs(frontier, explored)
                }
            }
        }
    }
}
pub fn solve_aystar(
    mut frontier: DoublePriorityQueue<Puzzle, usize>,
    mut explored: HashSet<Puzzle>,
    heut: Heust,
) -> Option<Solution> {
    match frontier.pop_min() {
        None => None,
        Some(node) => {
            explored.insert(node.clone().0);
            match node.clone().0.checkgoal() {
                true => Some(Solution::from_goal(
                    node.clone().0,
                    vec![node.0],
                    explored,
                    0,
                )),
                false => {
                    let parent = node.0.getchildren(heut.clone());
                    for mut child in parent.neighbours {
                        let mut dup = false;
                        match heut {
                            Heust::Mann => child = child.calc_mann(),
                            Heust::Eucl => child = child.calc_eucl(),
                            Heust::NoH => {}
                        }
                        for node in explored.clone() {
                            //Iterate over explored to prevent
                            //duplicate states
                            if child.clone().equals(node) {
                                dup = true;
                                break;
                            }
                        }
                        if !dup {
                            let h: usize =
                                child.clone().score.elements_row_major_iter().sum::<u8>() as usize;
                            child.cost = parent.cost + 1;
                            frontier.push(child.clone(), h + child.cost);
                            dbg!(child.state, h, child.cost);
                        }
                    }
                    solve_aystar(frontier, explored, heut)
                }
            }
        }
    }
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
