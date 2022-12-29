use chess::Board;
use crate::evaluation::evaluate;
use crate::search::move_order::get_move_order_captures;
use crate::search::NodeType;

pub fn quiesce_search_max(board: Board, mut alpha: i32, beta: i32) -> (i32, NodeType) {

    let stand_pat = evaluate(&board);

    if stand_pat >= beta { return (beta, NodeType::CUT); }
    if stand_pat > alpha { alpha = stand_pat; }

    let moves = get_move_order_captures(&board);

    if moves.len() == 0 { return (stand_pat, NodeType::PV) }

    let mut value = stand_pat;

    for joice in moves {

        let copy = board.make_move_new(joice);

        let (score, _) = quiesce_search_min(copy, alpha, beta);

        value = value.max(score);

        if value >= beta {
            return (beta, NodeType::ALL)
        }

        alpha = alpha.max(value);
    }

    (value, NodeType::PV)
}

pub fn quiesce_search_min(board: Board, alpha: i32, mut beta: i32) -> (i32, NodeType) {

    let stand_pat = evaluate(&board);

    if stand_pat <= alpha { return (alpha, NodeType::CUT); }
    if stand_pat < beta { beta = stand_pat; }

    let moves = get_move_order_captures(&board);

    if moves.len() == 0 { return (stand_pat, NodeType::PV) }

    let mut value = stand_pat;

    for joice in moves {

        let copy = board.make_move_new(joice);

        let (score, _) = quiesce_search_max(copy, alpha, beta);
        value = value.min(score);

        if value <= alpha {
            return (alpha, NodeType::ALL);
        }

        beta = beta.min(value);

    }

    (value, NodeType::PV)
}
