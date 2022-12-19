use std::sync::Arc;
use chess::{Board, ChessMove, EMPTY, MoveGen, Piece};
use crate::search::SearchData;

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
pub fn get_move_order(board: &Board, search_data: Arc<SearchData>) -> Vec<ChessMove> {
    let mut moves: Vec<ChessMove> = Vec::new();

    if let Some(pre_move) = search_data.best_moves.get(&board.get_hash()) {
        moves.push(*pre_move);
    }

    let mut all_moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    all_moves.set_iterator_mask(*targets);

    let mut moves_in_order = all_moves.by_ref().collect::<Vec<ChessMove>>();
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

    for joice in &moves_in_order {
        if !moves.contains(joice) {
            moves.push(*joice);
        }
    }

    all_moves.set_iterator_mask(!EMPTY);
    let second_half = all_moves.collect::<Vec<ChessMove>>();

    for joice in &second_half {
        if !moves.contains(joice) {
            moves.push(*joice);
        }
    }

    moves
}

pub fn get_move_order_captures(board: &Board) -> Vec<ChessMove> {
    let mut iterable = MoveGen::new_legal(&board);

    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    let mut targets: Vec<ChessMove> = iterable.collect();

    targets.sort_by(|a, b| {
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

    targets
}

#[test]
fn test_move_order() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES, MoveGen};

    let board = Board::from_str("rnbqkbnr/1p2p1pp/p4p2/8/3P4/4p1P1/PPP4P/RNBQKBNR b KQkq - 0 4").unwrap();

    let all_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

    let moves_in_order: Vec<ChessMove> = get_move_order(&board, Arc::new(SearchData::new()));

    // Check if both arrays are the same
    assert_eq!(all_moves.len(), moves_in_order.len());

    for joice in &all_moves {
        assert!(moves_in_order.contains(joice));
    }

}

#[test]
fn test_right_move_order() {
    use std::str::FromStr;
    use chess::{ChessMove, ALL_SQUARES, MoveGen};

    let board = Board::from_str("rnbqkbnr/1p2p1pp/p4p2/8/3P4/4p1P1/PPP4P/RNBQKBNR b KQkq - 0 4").unwrap();

    let all_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();

    let mut search_data = Arc::new(SearchData::new());
    search_data.best_moves.insert(board.get_hash(), *all_moves.last().unwrap());

    let moves_in_order: Vec<ChessMove> = get_move_order(&board, search_data);

    // Check if both arrays are the same
    assert_eq!(all_moves.len(), moves_in_order.len());

    for joice in &all_moves {
        assert!(moves_in_order.contains(joice));
    }

    // check if best move first approach works
    assert_eq!(all_moves.last(), moves_in_order.first())
}