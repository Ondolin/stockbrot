use std::cmp::Ordering;
use chess::{ALL_SQUARES, Board, Color, Piece, Square};

pub fn evaluate(board: &Board) -> f32 {

    score_board(board)

}

fn score_board(board: &Board) -> f32 {
    let b = board.color_combined(Color::White).0;
    let mut sum = 0f32;

    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let a: Square = ALL_SQUARES[i as usize];
            sum += match board.piece_on(a).unwrap() {
                Piece::Pawn => {1.0},
                Piece::Knight => {3.0},
                Piece::Bishop => {3.0},
                Piece::Rook => {5.0},
                Piece::Queen => {9.0},
                Piece::King => {100.0},
            }

        }
    }

    let b = board.color_combined(Color::Black).0;

    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let a: Square = ALL_SQUARES[i as usize];
            sum -= match board.piece_on(a).unwrap() {
                Piece::Pawn => {1.0},
                Piece::Knight => {3.0},
                Piece::Bishop => {3.0},
                Piece::Rook => {5.0},
                Piece::Queen => {9.0},
                Piece::King => {100.0},
            }

        }
    }

    sum
}