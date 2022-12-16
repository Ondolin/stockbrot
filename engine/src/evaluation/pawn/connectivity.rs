use chess::{BitBoard, Color, get_file, Square};
use crate::evaluation::pawn::RANK_BLOCKS;

const BONUS_SEED: [i32; 7] = [0, 7, 8, 12, 29, 48, 86];

// returns the number of pawns protecting us
fn supporter(all_my_pawns: &BitBoard, field: Square, color: Color) -> i32 {
    let mut counter = 0;

    // move to row of supporters
    let field = if color == Color::White {
        field.down().expect("There can not be a pawn on Rank 1")
    } else {
        field.up().expect("There can not be a pawn on Rank 8")
    };

    // check left and right
    for field in [field.left(), field.right()] {
        // ignore if the pawn is an outer pawn
        if let Some(field) = field {
            if all_my_pawns & BitBoard::from_square(field) != BitBoard(0) {
                counter += 1;
            }
        }
    }

    counter
}

// return 1 if we have a pawn next to us
fn phalanx(all_my_pawns: &BitBoard, field: Square) -> i32 {

    // check left and right
    for field in [field.left(), field.right()] {
        // ignore if the pawn is an outer pawn
        if let Some(field) = field {
            if all_my_pawns & BitBoard::from_square(field) != BitBoard(0) {
                return 1;
            }
        }
    }

    0
}

fn opposed(all_opponent_pawns: &BitBoard, field: Square, color: Color) -> i32 {
    let block = if color == Color::White {
        !RANK_BLOCKS[field.get_rank().to_index()]
    } else {
        RANK_BLOCKS[field.get_rank().to_index()]
    };

    if all_opponent_pawns & get_file(field.get_file()) & block != BitBoard(0) {
        return 1;
    }

    0
}


pub fn connected_bonus(all_my_pawns: &BitBoard, all_other_pawns: &BitBoard, field: Square, color: Color) -> i32 {
    let supporter = supporter(all_my_pawns, field, color);
    let phalanx = phalanx(all_my_pawns, field);

    if supporter == 0 && phalanx == 0 { return 0; }

    let opposed = opposed(&all_other_pawns, field, color);

    let transposed_rank = if color == Color::White {
        field.get_rank().to_index()
    } else {
        7 - field.get_rank().to_index()
    };

    BONUS_SEED[transposed_rank] * (2 + phalanx - opposed) + 21 * supporter
}

#[test]
fn test_supporter() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES};

    let board = Board::from_str("rnbqkbnr/1p2p1pp/p4p2/8/3P4/2P1p3/PP4PP/RNBQKBNR w KQkq - 0 4").unwrap();

    let mut w_score = 0;
    let mut b_score = 0;

    for i in 0..64 {
        if let Some(piece) = board.piece_on(ALL_SQUARES[i as usize]) {

            if piece != Piece::Pawn { continue; }

            let color = board.color_on(ALL_SQUARES[i as usize]).unwrap();

            let all_pawns = board.pieces(Piece::Pawn) & board.color_combined(color);

            if color == Color::White {
                w_score += supporter(&all_pawns, ALL_SQUARES[i as usize], color);
            } else {
                b_score += supporter(&all_pawns, ALL_SQUARES[i as usize], color);
            }

        }
    }

    assert_eq!(w_score, 2);
    assert_eq!(b_score, 3);
}

#[test]
fn test_phalanx() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES};

    let board = Board::from_str("rnbqkbnr/1p2p1pp/p4p2/8/3P4/4p1P1/PPP4P/RNBQKBNR b KQkq - 0 4").unwrap();

    let mut w_score = 0;
    let mut b_score = 0;

    for i in 0..64 {
        if let Some(piece) = board.piece_on(ALL_SQUARES[i as usize]) {

            if piece != Piece::Pawn { continue; }

            let color = board.color_on(ALL_SQUARES[i as usize]).unwrap();

            let all_pawns = board.pieces(Piece::Pawn) & board.color_combined(color);

            if color == Color::White {
                w_score += phalanx(&all_pawns, ALL_SQUARES[i as usize]);
            } else {
                b_score += phalanx(&all_pawns, ALL_SQUARES[i as usize]);
            }

        }
    }

    assert_eq!(w_score, 3);
    assert_eq!(b_score, 2);
}

#[test]
fn test_opposed() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES};

    let board = Board::from_str("rnb1kbnr/1p2pp1p/p5p1/8/8/P1P5/P2PP1PP/RNBQKBNR w KQkq - 0 2").unwrap();

    let mut w_score = 0;
    let mut b_score = 0;

    for i in 0..64 {
        if let Some(piece) = board.piece_on(ALL_SQUARES[i as usize]) {

            if piece != Piece::Pawn { continue; }

            let color = board.color_on(ALL_SQUARES[i as usize]).unwrap();

            let all_pawns = board.pieces(Piece::Pawn) & board.color_combined(!color);

            if color == Color::White {
                w_score += opposed(&all_pawns, ALL_SQUARES[i as usize], color);
            } else {
                b_score += opposed(&all_pawns, ALL_SQUARES[i as usize], color);
            }

        }
    }

    assert_eq!(w_score, 5);
    assert_eq!(b_score, 4);
}

#[test]
fn test_bonus() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES};

    let board = Board::from_str("rnb1kbnr/2P1pp1p/1P1P2p1/1p6/2p5/8/3P1PPP/RNBQKBNR b KQkq - 0 2").unwrap();

    let mut w_score = 0;
    let mut b_score = 0;

    for i in 0..64 {
        if let Some(piece) = board.piece_on(ALL_SQUARES[i as usize]) {

            if piece != Piece::Pawn { continue; }

            let color = board.color_on(ALL_SQUARES[i as usize]).unwrap();

            let all_other_pawns = board.pieces(Piece::Pawn) & board.color_combined(!color);
            let all_my_pawns = board.pieces(Piece::Pawn) & board.color_combined(color);

            if color == Color::White {
                w_score += connected_bonus(&all_my_pawns, &all_other_pawns, ALL_SQUARES[i as usize], color);
            } else {
                b_score += connected_bonus(&all_my_pawns, &all_other_pawns, ALL_SQUARES[i as usize], color);
            }

        }
    }

    assert_eq!(w_score, 256);
    assert_eq!(b_score, 164);
}