use std::sync::{Arc, RwLock};
use std::sync::atomic::Ordering;
use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::Engine;
use crate::search::evaluated_position::{EvaluatedPositions, EvaluatedPositionsFunctions};
use crate::search::quiesce_search::{quiesce_search_max, quiesce_search_min};
use crate::search::move_order::get_move_order;

use rayon::prelude::*;
use crate::search::{SearchData, STOP_THREADS};

impl Engine {
    pub fn alpha_beta_search(&self, max_depth: u8, search_data: Arc<SearchData>) -> (Option<ChessMove>, i32) {

        // dbg!(&search_data.positions_visited);

        let best_move: RwLock<(Option<ChessMove>, i32)> =
            RwLock::new((
                None,
                if self.game.side_to_move() == Color::White { i32::MIN } else { i32::MAX }
            ));

        let moves = MoveGen::new_legal(&self.game.current_position()).collect::<Vec<ChessMove>>();
        moves.par_iter().for_each(|joice| {

            if STOP_THREADS.load(Ordering::SeqCst) { return; }


            let copy = self.game.current_position().make_move_new(*joice);

            // prevent repetition of moves
            if self.search_data.position_visited_twice(&copy) {
                log::info!("Can not play {} due to repetition of moves", joice.to_string());
                return;
            }

            let alpha = i32::MIN;
            let beta = i32::MAX;

            if self.game.side_to_move() == Color::White {

                let score = alpha_beta_min(copy, alpha, beta, max_depth - 1, search_data.clone());
                if STOP_THREADS.load(Ordering::SeqCst) { return; }

                log::info!("Move Evaluation: {} {score}", joice.to_string());

                if score >= { best_move.read().unwrap().1 } {
                    *best_move.write().unwrap() = (Some(*joice), score);
                }
            } else {
                let score = alpha_beta_max(copy, alpha, beta, max_depth - 1, search_data.clone());
                if STOP_THREADS.load(Ordering::SeqCst) { return; }

                log::info!("Move Evaluation: {} {score}", joice.to_string());

                if score <= { best_move.read().unwrap().1 } {
                    *best_move.write().unwrap() = (Some(*joice), score);
                }
            }

        });

        let best_move = *best_move.read().unwrap();

        best_move
    }

}

pub fn alpha_beta_max(board: Board, mut alpha: i32, beta: i32, depth_left: u8, search_data: Arc<SearchData>) -> i32 {
    // leaf node
    /*if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| {
        quiesce_search_max(board, i32::MIN, i32::MAX)
    } ) }*/
    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_max(board, alpha, beta);
    }

    let mut value = i32::MIN;

    for joice in get_move_order(&board) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);

        let score = search_data
            .get_or_calculate_evaluation(
                copy,
                depth_left,
                |eval| alpha_beta_min(copy, alpha, beta, depth_left - 1, eval)
            );

        value = value.max(score);

        if value >= beta { break }

        alpha = alpha.max(value);

    }

    value
}

pub fn alpha_beta_min(board: Board, alpha: i32, mut beta: i32, depth_left: u8, search_data: Arc<SearchData>) -> i32 {
    // leaf node
    /*if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| {
        quiesce_search_min(board, i32::MIN, i32::MAX)
    } ) }*/

    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_min(board, alpha, beta);
    }

    let mut value = i32::MAX;

    for joice in get_move_order(&board) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);
        value = value.min(search_data
            .get_or_calculate_evaluation(
                copy,
                depth_left,
                |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));

        if value <= alpha { break }

        beta = beta.min(value);
    }

    value
}