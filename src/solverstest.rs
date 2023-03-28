use std::collections::{HashSet, VecDeque};

use crate::{
    init_puz,
    solvers::{solve_bfs, solve_dfs},
    Puzzle,
};

#[test]
fn dfs() {
    let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows);
    let mut vec_q: VecDeque<Puzzle> = VecDeque::new();
    vec_q.push_back(test_puzzle.clone());
    let solly = solve_dfs(vec_q, HashSet::new()).expect("Nope");
    assert!(solly.get_path().first().expect("??").clone().checkgoal())
}
#[test]
fn bfs() {
    let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows);
    let mut vec_q: VecDeque<Puzzle> = VecDeque::new();
    vec_q.push_back(test_puzzle.clone());
    let solly = solve_bfs(vec_q, HashSet::new()).expect("Nope");
    assert!(solly.get_path().first().expect("??").clone().checkgoal())
}
