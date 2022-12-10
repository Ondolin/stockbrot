use test::Bencher;

use chess::{ALL_SQUARES, Board, BoardStatus, Color};
use crate::evaluation::{game_phase_inc, MATE_SCORE, mg_value, eg_value, piece_mobility};

pub fn evaluate(board: &Board) -> i32 {

    let game_status = board.status();
    if game_status == BoardStatus::Stalemate { return 0 }
    if game_status == BoardStatus::Checkmate { return if board.side_to_move() == Color::Black { MATE_SCORE } else { -MATE_SCORE } }

    score_board(board)

}

const fn color_multiplier(color: &Color) -> i32 {
    match color {
        Color::White => 1,
        Color::Black => -1
    }
}

fn score_board(board: &Board) -> i32 {
    // sum in mid game
    let mut mg_score: i32 = 0;

    // sum in end game
    let mut eg_score: i32 = 0;

    // gamephase is determined by the amount of pieces present
    let mut game_phase = 0;

    let bitboard = board.combined();
    let b = bitboard.0;

    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let square = ALL_SQUARES[i as usize];

            let piece = board.piece_on(square).unwrap();
            let color = board.color_on(square).unwrap();

            // Piece and position values
            mg_score += mg_value(&piece,  color, i as u8);
            eg_score += eg_value(&piece, color, i as u8);

            // Mobility bonus

            let (mg_piece_mobility, eg_piece_mobility) = piece_mobility(&piece, *bitboard, square);

            mg_score += color_multiplier(&color) * mg_piece_mobility;
            eg_score += color_multiplier(&color) * eg_piece_mobility;

            game_phase += game_phase_inc(&piece);
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

    let board = Board::from_str("3q1rk1/5ppp/2n2n2/p1pNb3/3pP3/3P3N/PPbB2PP/R3KB1R b KQ - 1 16").unwrap();
    //let board = board.make_move_new(ChessMove::from_str("a6a8").unwrap());

    b.iter(|| {
        evaluate(&board);
    });
}