use array2d::Array2D;

use crate::{Direction, Puzzle};

#[test]
fn move_test() {
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows, (1, 0));
    assert_eq!((1, 0), test_puzzle.clone().getzero());

    (test_puzzle.state, test_puzzle.zeropos) = test_puzzle.clone().move_zero(Direction::Down);
    assert_eq!((1, 1), test_puzzle.clone().getzero());

    (test_puzzle.state, test_puzzle.zeropos) = test_puzzle.clone().move_zero(Direction::Right);
    assert_eq!((2, 1), test_puzzle.clone().getzero());

    (test_puzzle.state, test_puzzle.zeropos) = test_puzzle.clone().move_zero(Direction::Left);
    assert_eq!((1, 1), test_puzzle.clone().getzero());

    (test_puzzle.state, test_puzzle.zeropos) = test_puzzle.clone().move_zero(Direction::Up);
    assert_eq!((1, 0), test_puzzle.clone().getzero());
}
#[test]
fn getmove_test() {
    let rows = vec![vec![0, 5, 2], vec![3, 1, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (0, 0));
    let moves = test_puzzle.getmoves();

    assert_eq!(2, moves.len());
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (1, 0));
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![2, 5, 0], vec![3, 1, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (2, 0));
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());

    let rows = vec![vec![1, 5, 2], vec![0, 3, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (0, 1));
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (1, 1));
    let moves = test_puzzle.getmoves();
    assert_eq!(4, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 0], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows, (2, 1));
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![0, 6, 7]];
    let test_puzzle = init_puz(rows, (0, 2));
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 0, 7]];
    let test_puzzle = init_puz(rows, (1, 2));
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows, (2, 2));
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());
}
fn init_puz(rows: Vec<Vec<u8>>, (zx, zy): (usize, usize)) -> Puzzle {
    let test_state: Array2D<u8> = Array2D::from_rows(&rows).expect("no");
    Puzzle {
        neighbours: vec![],
        parent: None,
        state: test_state,
        zeropos: (zx, zy),
    }
}
