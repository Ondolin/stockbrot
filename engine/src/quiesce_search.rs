use chess::{Board, MoveGen};
use crate::evaluate::evaluate;

pub fn quiesce_search_max(board: Board, mut alpha: i32, mut beta: i32) -> i32 {

    let mut iterable = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    if iterable.len() == 0 { return evaluate(&board) }

    let mut value = evaluate(&board);

    for joice in iterable {

        let copy = board.make_move_new(joice);

        let score = quiesce_search_min(copy, alpha, beta);

        value = value.max(score);

        if value >= beta {
            break;
        }

        alpha = alpha.max(value);
    }

    value
}

pub fn quiesce_search_min(board: Board, mut alpha: i32, mut beta: i32) -> i32 {

    let mut iterable = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    iterable.set_iterator_mask(*targets);

    if iterable.len() == 0 { return evaluate(&board) }

    let mut value = evaluate(&board);

    for joice in iterable {

        let copy = board.make_move_new(joice);

        value = value.min(quiesce_search_max(copy, alpha, beta));

        if value <= alpha {
            break;
        }

        beta = beta.min(value);

    }

    value
}
