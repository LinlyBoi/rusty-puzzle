use std::collections::HashSet;

mod puzzlin;
use array2d::Array2D;
use priority_queue::DoublePriorityQueue;

use crate::puzzlin::init_puz;

fn main() {
    // let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
    let rows = vec![vec![1, 4, 2], vec![3, 0, 8], vec![6, 5, 7]];
    // let test_puzzle = init_puz(rows);
    // let mut vec_q: VecDeque<Puzzle> = VecDeque::new();
    // vec_q.push_back(test_puzzle.clone());
    // let solly = solve_dfs(vec_q, HashSet::new()).expect("Nope");
    // let solly = solve_bfs(vec_q, HashSet::new()).expect("BFS is sucks");
    let test_puzzle = init_puz(rows).calc_mann();
    let init_h: usize = test_puzzle
        .getscore()
        .elements_row_major_iter()
        .sum::<u8>()
        .into();
    let mut pq: DoublePriorityQueue<Puzzle, usize> = DoublePriorityQueue::new();
    pq.push(test_puzzle, init_h);
    let solly = solve_aystar(pq, HashSet::new(), crate::heuristic::Heust::Mann).expect("Nope");
    for step in solly.clone().get_path() {
        dbg!("funny path:", step.state);
    }
    dbg!(solly.clone().get_cost());
    for exp in solly.get_explored() {
        dbg!(exp.cost);
    }
}
