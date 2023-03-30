use std::collections::{HashSet, VecDeque};

use priority_queue::DoublePriorityQueue;

use crate::puzzlin::{
    heuristic::Heust,
    init_puz,
    solvers::{solve_aystar, solve_bfs, solve_dfs},
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
    vec_q.push_back(test_puzzle);
    let solly = solve_bfs(vec_q, HashSet::new()).expect("Nope");
    assert!(solly.get_path().first().expect("??").clone().checkgoal())
}
#[test]
fn aystar_mann() {
    // let rows = vec![vec![1, 8, 2], vec![0, 4, 3], vec![7, 6, 5]];
    let rows = vec![vec![1, 4, 2], vec![3, 0, 8], vec![6, 5, 7]];
    let test_puzzle = init_puz(rows).calc_mann();
    let init_h: usize = test_puzzle
        .score
        .elements_row_major_iter()
        .sum::<u8>()
        .into();
    let mut pq: DoublePriorityQueue<Puzzle, usize> = DoublePriorityQueue::new();
    pq.push(test_puzzle, init_h);
    let solly = solve_aystar(pq, HashSet::new(), Heust::Mann).expect("Nope");
    assert!(solly.get_path().first().expect("??").clone().checkgoal())
}
#[test]
fn aystar_eucl() {
    let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows).calc_mann();
    let init_h: usize = test_puzzle
        .score
        .elements_row_major_iter()
        .sum::<u8>()
        .into();
    let mut pq: DoublePriorityQueue<Puzzle, usize> = DoublePriorityQueue::new();
    pq.push(test_puzzle, init_h);
    let solly = solve_aystar(pq, HashSet::new(), Heust::Eucl).expect("Nope");
    assert!(solly.get_path().first().expect("??").clone().checkgoal())
}
