use chess::{Board, ChessMove, EMPTY, MoveGen, Piece};

fn piece_type(p: &Piece) -> u8 {
    match p {
        Piece::Pawn => 1,
        Piece::Knight => 2,
        Piece::Bishop => 3,
        Piece::Rook => 4,
        Piece::Queen => 5,
        Piece::King => 10
    }
}

// put captures before other moves
pub fn get_move_order(board: &Board) -> Vec<ChessMove> {
    let mut moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);

    let mut moves_in_order = moves.by_ref().collect::<Vec<ChessMove>>();
    moves_in_order.sort_by(|a, b| {
        let a_type = board.piece_on(a.get_source()).unwrap();
        let b_type = board.piece_on(b.get_source()).unwrap();

        // If a is a lesser piece use it first
        if piece_type(&a_type) < piece_type(&b_type) { return core::cmp::Ordering::Greater }
        else if piece_type(&a_type) > piece_type(&b_type) { return core::cmp::Ordering::Less }

        let a_target = board.piece_on(a.get_dest()).unwrap();
        let b_target = board.piece_on(b.get_dest()).unwrap();

        // Capture high value pieces first
        if piece_type(&a_target) > piece_type(&b_target) { return core::cmp::Ordering::Greater }
        else if piece_type(&a_target) < piece_type(&b_target) { return core::cmp::Ordering::Less }

        core::cmp::Ordering::Equal
    });

    moves.set_iterator_mask(!EMPTY);
    let mut second_half = moves.collect::<Vec<ChessMove>>();
    moves_in_order.append(&mut second_half);

    moves_in_order
}