use array2d::Array2D;

use crate::Puzzle;

#[test]
fn move_test() {
    let mut test_puzzle = init_puz();
    assert_eq!((1, 0), test_puzzle.clone().getzero());
    test_puzzle.state = test_puzzle.clone().move_zero(crate::Direction::Down);
    assert_eq!((2, 0), test_puzzle.getzero());
}
fn init_puz() -> Puzzle {
    let test_rows: Vec<Vec<u8>> = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let test_state: Array2D<u8> = Array2D::from_rows(&test_rows).expect("no");
    Puzzle {
        neighbours: vec![],
        parent: None,
        state: test_state,
        zeropos: (1, 0),
    }
}
