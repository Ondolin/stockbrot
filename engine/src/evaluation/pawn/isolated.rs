use chess::{BitBoard, File, get_file, Square};

pub fn isolated(all_my_pawns: &BitBoard, field: Square) -> bool {
    let own_file = field.get_file();

    let mut mask = BitBoard(0);

    if own_file != File::A {
        mask |= get_file(own_file.left());
    }
    if own_file != File::H {
        mask |= get_file(own_file.right());
    }

    all_my_pawns & mask == BitBoard(0)
}

pub fn double_isolated(all_my_pawns: &BitBoard, field: Square) -> i32 {
    if !isolated(all_my_pawns, field) { return 0 }

    (all_my_pawns & get_file(field.get_file())).count() as i32
}


#[test]
fn test_isolated() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES, Board, Piece, Color};

    let board = Board::from_str("rnb1kbnr/2P1pp1p/1P1P2p1/p7/2p5/8/3P1PPP/RNBQKBNR b KQkq - 0 2").unwrap();

    let white_pawns = board.pieces(Piece::Pawn) & board.color_combined(Color::White);
    let black_pawns = board.pieces(Piece::Pawn) & board.color_combined(Color::Black);

    assert!(isolated(&black_pawns, Square::A5));
    assert!(isolated(&black_pawns, Square::C4));

    assert!(!isolated(&white_pawns, Square::C7));

}