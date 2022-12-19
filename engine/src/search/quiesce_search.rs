use chess::Board;
use crate::evaluation::evaluate;
use crate::search::move_order::get_move_order_captures;

pub fn quiesce_search_max(board: Board, mut alpha: i32, beta: i32) -> i32 {

    let stand_pat = evaluate(&board);
    if stand_pat >= beta { return beta; }
    if stand_pat > alpha { alpha = stand_pat; }

    let moves = get_move_order_captures(&board);

    if moves.len() == 0 { return evaluate(&board) }

    let mut value = evaluate(&board);

    for joice in moves {

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

pub fn quiesce_search_min(board: Board, alpha: i32, mut beta: i32) -> i32 {

    let stand_pat = evaluate(&board);
    if stand_pat <= alpha { return alpha; }
    if stand_pat < beta { beta = stand_pat; }

    let moves = get_move_order_captures(&board);

    if moves.len() == 0 { return evaluate(&board) }

    let mut value = evaluate(&board);

    for joice in moves {

        let copy = board.make_move_new(joice);

        value = value.min(quiesce_search_max(copy, alpha, beta));

        if value <= alpha {
            break;
        }

        beta = beta.min(value);

    }

    value
}
