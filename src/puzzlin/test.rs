use std::collections::HashSet;

use array2d::Array2D;

use crate::{find_index, init_puz, solvers::Solution, Direction, Heust};

#[test]
fn move_test_down() {
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows);

    test_puzzle = test_puzzle.clone().move_zero(Direction::Down);
    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let current_puzzle = init_puz(rows);
    assert!(current_puzzle.equals(test_puzzle.clone()));
    test_puzzle = test_puzzle.clone().move_zero(Direction::Down);
    let rows = vec![vec![1, 5, 2], vec![3, 6, 4], vec![8, 0, 7]];
    let current_puzzle = init_puz(rows);
    assert!(current_puzzle.equals(test_puzzle));
}

#[test]
fn move_test_up() {
    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows);

    test_puzzle = test_puzzle.clone().move_zero(Direction::Up);
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let current_puzzle = init_puz(rows);
    assert!(current_puzzle.equals(test_puzzle));
}
#[test]
fn move_test_left() {
    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows);

    test_puzzle = test_puzzle.clone().move_zero(Direction::Left);
    let rows = vec![vec![1, 5, 2], vec![0, 3, 4], vec![8, 6, 7]];
    let current_puzzle = init_puz(rows);
    assert!(current_puzzle.equals(test_puzzle));
}
#[test]
fn move_test_right() {
    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows);

    test_puzzle = test_puzzle.clone().move_zero(Direction::Right);
    let rows = vec![vec![1, 5, 2], vec![3, 4, 0], vec![8, 6, 7]];
    let current_puzzle = init_puz(rows);
    assert!(current_puzzle.equals(test_puzzle));
}

#[test]
fn getmove_test() {
    let rows = vec![vec![0, 5, 2], vec![3, 1, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();

    assert_eq!(2, moves.len());
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![2, 5, 0], vec![3, 1, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());

    let rows = vec![vec![1, 5, 2], vec![0, 3, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(4, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 0], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![0, 6, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 0, 7]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(3, moves.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows);
    let moves = test_puzzle.getmoves();
    assert_eq!(2, moves.len());
}

#[test]
fn getchildren_test() {
    let rows = vec![vec![0, 5, 2], vec![3, 1, 4], vec![8, 6, 7]];
    let mut test_puzzle = init_puz(rows);
    test_puzzle = test_puzzle.clone().getchildren(Heust::NoH);

    let test_rows = vec![vec![3, 5, 2], vec![0, 1, 4], vec![8, 6, 7]];
    let test2_puzzle = init_puz(test_rows);
    let test_rows = vec![vec![5, 0, 2], vec![3, 1, 4], vec![8, 6, 7]];
    let test3_puzzle = init_puz(test_rows);
    let mut matching = 0;
    for child in test_puzzle.clone().neighbours {
        if child.clone().equals(test2_puzzle.clone()) {
            matching += 1;
        }
        if child.equals(test3_puzzle.clone()) {
            matching += 1;
        }
    }
    assert_eq!(2, matching);

    assert_eq!(2, test_puzzle.neighbours.len());
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(3, children.neighbours.len());

    let rows = vec![vec![2, 5, 0], vec![3, 1, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(2, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![0, 3, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(3, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![3, 0, 4], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(4, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 0], vec![8, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(3, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![0, 6, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(2, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 0, 7]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(3, children.neighbours.len());

    let rows = vec![vec![1, 5, 2], vec![3, 4, 8], vec![6, 7, 0]];
    let test_puzzle = init_puz(rows);
    let children = test_puzzle.getchildren(Heust::NoH);
    assert_eq!(2, children.neighbours.len());
}
#[test]
fn checkgoal() {
    let rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let goal = init_puz(rows);
    assert_eq!(None, goal.parent);
    assert!(goal.checkgoal())
}
#[test]
fn equals() {
    let rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let p1 = init_puz(rows.clone());
    let p2 = init_puz(rows);
    assert!(p1.equals(p2))
}

#[test]
fn twodee() {
    let rows = vec![vec![1, 0, 2], vec![3, 5, 4], vec![8, 6, 7]];
    let mut twod = Array2D::from_rows(&rows).expect("??");
    assert_eq!(1, twod[(0, 0)]);
    twod[(0, 0)] = 10;
    assert_eq!(10, twod[(0, 0)]);
    assert_eq!(0, twod[(0, 1)]);
    assert_eq!(10, twod[(0, 1 - 1)]);
    assert_eq!(2, twod[(0, 2)]);
    assert_eq!((0, 0), find_index(twod.clone(), 10).expect("AA"));
    let temp = twod[(1, 0)];
    twod[(1, 0)] = twod[(2, 1)];
    twod[(2, 1)] = temp;
    assert_eq!(twod[(1, 0)], 6);
    assert_eq!(twod[(2, 1)], 3);
}
#[test]
fn solution_creation() {
    let rows = vec![vec![0, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
    let p1 = init_puz(rows);
    _ = Solution::from_goal(p1, vec![], HashSet::new(), 0)
}
