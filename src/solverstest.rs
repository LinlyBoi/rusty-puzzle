use crate::{init_puz, solvers::solve_dfs};

// #[test]
// fn dfs() {
//     let rows = vec![vec![5, 2, 8], vec![4, 1, 7], vec![0, 3, 6]];
//     let test_puzzle = init_puz(rows, (0, 2));
//     let solly = solve_dfs(vec![test_puzzle], vec![]).expect("Nope");
//     assert!(solly.get_path().pop().expect("??").checkgoal())
// }
