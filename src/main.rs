use std::collections::HashSet;

use priority_queue::DoublePriorityQueue;

use rusting_puzzle::puzzlin::{heuristic::Heust, init_puz, solvers::solve_aystar, Puzzle};

fn main() -> eframe::Result<()> {
    let rows = vec![vec![1, 4, 2], vec![3, 0, 8], vec![6, 5, 7]];
    // let test_puzzle = init_puz(rows);
    // let mut vec_q: VecDeque<Puzzle> = VecDeque::new();
    // vec_q.push_back(test_puzzle.clone());
    // let solly = solve_dfs(vec_q, HashSet::new()).expect("Nope");
    // let solly = solve_bfs(vec_q, HashSet::new()).expect("BFS is sucks");
    let test_puzzle = init_puz(rows).calc_mann();
    let init_h: usize = test_puzzle
        .clone()
        .getscore()
        .elements_row_major_iter()
        .sum::<u8>()
        .into();
    let mut pq: DoublePriorityQueue<Puzzle, usize> = DoublePriorityQueue::new();
    pq.push(test_puzzle, init_h);
    let solly = solve_aystar(pq, HashSet::new(), Heust::Mann).expect("Nope");

    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Rusty Puzzle Solver",
        native_options,
        Box::new(|cc| Box::new(rusting_puzzle::RustyPuzzle::new(cc))),
    )
    // let rows = vec![vec![1, 4, 2], vec![3, 5, 8], vec![6, 7, 0]];
}
