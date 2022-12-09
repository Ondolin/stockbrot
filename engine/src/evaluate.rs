use std::cmp::Ordering;
use chess::{ALL_SQUARES, Board, BoardStatus, Color, Piece, Square};

pub const MATE_SCORE: i32 = 1_000_000;

pub fn evaluate(board: &Board) -> i32 {

    if board.status() == BoardStatus::Stalemate { return 0 }
    if board.status() == BoardStatus::Checkmate { return if board.side_to_move() == Color::Black { MATE_SCORE } else { -MATE_SCORE } }

    score_board(board)

}

fn score_board(board: &Board) -> i32 {
    let b = board.color_combined(Color::White).0;
    let mut sum: i32 = 0;

    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let a: Square = ALL_SQUARES[i as usize];
            sum += match board.piece_on(a).unwrap() {
                Piece::Pawn => {100},
                Piece::Knight => {300},
                Piece::Bishop => {300},
                Piece::Rook => {500},
                Piece::Queen => {900},
                Piece::King => {10_000},
            }

        }
    }

    let b = board.color_combined(Color::Black).0;

    for i in 0..64u64 {
        if b & (1 << i) != 0 {
            let a: Square = ALL_SQUARES[i as usize];
            sum -= match board.piece_on(a).unwrap() {
                Piece::Pawn => {100},
                Piece::Knight => {300},
                Piece::Bishop => {300},
                Piece::Rook => {500},
                Piece::Queen => {900},
                Piece::King => {10_000},
            }

        }
    }

    assert!(sum.abs() < 50_000);

    sum
}

fn get_value_by_square(board: &Board, square: Square) -> f32 {
    let Some(piece) = board.piece_on(square) else { return 0.0; };
    let color = board.color_on(square).unwrap();

    let mut sum = 0.0;

    match piece {
        Piece::Pawn => 1.0,
        Piece::Knight => 3.0,
        Piece::Bishop => 3.1,
        Piece::Rook => 5.0,
        Piece::Queen => 9.0,
        Piece::King => 100.0,
    };

    sum * if color == Color::White { 1.0 } else { -1.0 }

}