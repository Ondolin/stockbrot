use test::Bencher;

use chess::{ALL_SQUARES, Board, BoardStatus, Color};
use crate::evaluation::{game_phase_inc, MATE_SCORE, mg_value, eg_value};

pub fn evaluate(board: &Board) -> i32 {

    let game_status = board.status();
    if game_status == BoardStatus::Stalemate { return 0 }
    if game_status == BoardStatus::Checkmate { return if board.side_to_move() == Color::Black { MATE_SCORE } else { -MATE_SCORE } }

    score_board(board)

}

fn score_board(board: &Board) -> i32 {
    // sum of all piece values + position values in mid game
    let mut mg_score: i32 = 0;

    // sum of all piece values + position values in end game
    let mut eg_score: i32 = 0;

    // gamephase is determined by the amount of pieces present
    let mut game_phase = 0;

    let b = board.combined().0;
    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let square = ALL_SQUARES[i as usize];

            let p = board.piece_on(square).unwrap();
            let c = board.color_on(square).unwrap();

            let i = i as u8;

            mg_score += mg_value(&p,  c, i);
            eg_score += eg_value(&p, c, i);

            game_phase += game_phase_inc(&p);
        }
    }

    let mut mg_phase = game_phase;
    if mg_phase > 24 { mg_phase = 24 }

    let eg_phase = 24 - mg_phase;

    (mg_score * mg_phase + eg_score * eg_phase) / 24

}

#[bench]
fn evaluation_speed(b: &mut Bencher) {
    use std::str::FromStr;
    use chess::ChessMove;

    let board = Board::from_str("7k/1R6/R7/8/8/8/8/K7 w - - 0 1").unwrap();
    let board = board.make_move_new(ChessMove::from_str("a6a8").unwrap());

    b.iter(|| {
        evaluate(&board);
    });
}