use chess::{ALL_SQUARES, Board, BoardStatus, Color};
use crate::evaluation::{game_phase_inc, MATE_SCORE, mg_value, eg_value};

pub fn evaluate(board: &Board) -> i32 {

    if board.status() == BoardStatus::Stalemate { return 0 }
    if board.status() == BoardStatus::Checkmate { return if board.side_to_move() == Color::Black { MATE_SCORE } else { -MATE_SCORE } }

    score_board(board)

}

fn score_board(board: &Board) -> i32 {
    // sum of all piece values + position values in mid game
    let mut mg_score: i32 = 0;

    // sum of all piece values + position values in end game
    let mut eg_score: i32 = 0;

    // gamephase is determined by the amount of pieces present
    let mut game_phase = 0;

    for i in 0..64u8 {
        let pos = ALL_SQUARES[i as usize];
        if let Some(piece) = board.piece_on(pos) {
            mg_score += mg_value(&board.piece_on(pos).unwrap(), board.color_on(pos).unwrap(), i);
            eg_score += eg_value(&board.piece_on(pos).unwrap(), board.color_on(pos).unwrap(), i);

            game_phase += game_phase_inc(&piece);
        }
    }

    let mut mg_phase = game_phase;
    if mg_phase > 24 { mg_phase = 24 }

    let eg_phase = 24 - mg_phase;

    (mg_score * mg_phase + eg_score * eg_phase) / 24

}