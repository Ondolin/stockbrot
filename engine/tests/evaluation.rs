use engine::Engine;
use engine::evaluation::{evaluate, MATE_SCORE};
use chess::{Board, ChessMove};
use std::str::FromStr;

#[test]
fn test_mate() {

    let board = Board::from_str("7k/1R6/R7/8/8/8/8/K7 w - - 0 1").unwrap();
    let board = board.make_move_new(ChessMove::from_str("a6a8").unwrap());

    assert_eq!(evaluate(&board), MATE_SCORE);
}