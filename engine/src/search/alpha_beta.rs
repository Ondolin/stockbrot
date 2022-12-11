use std::hash::Hash;
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

        {
            search_data.best_moves.clear();
        }

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
    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_max(board, alpha, beta);
    }

    let mut value = i32::MIN;
    let mut best_move: Option<ChessMove> = None;

    for joice in get_move_order(&board, search_data.clone()) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);

        let score = search_data
            .get_or_calculate_evaluation(
                copy,
                depth_left,
                |eval| alpha_beta_min(copy, alpha, beta, depth_left - 1, eval)
            );

        if score > value {
            value = score;
            best_move = Some(joice);
        }

        if value >= beta { break }

        alpha = alpha.max(value);

    }

    if let Some(best_move) = best_move {
        search_data.best_moves.insert(board.get_hash(), best_move);
    }

    value
}

pub fn alpha_beta_min(board: Board, alpha: i32, mut beta: i32, depth_left: u8, search_data: Arc<SearchData>) -> i32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_min(board, alpha, beta);
    }

    let mut value = i32::MAX;
    let mut best_move: Option<ChessMove> = None;

    for joice in get_move_order(&board, search_data.clone()) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);
        let score = search_data
            .get_or_calculate_evaluation(
                copy,
                depth_left,
                |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval));

        if score < value {
            value = score;
            best_move = Some(joice);
        }

        if value <= alpha { break }

        beta = beta.min(value);
    }

    if let Some(best_move) = best_move {
        search_data.best_moves.insert(board.get_hash(), best_move);
    }

    value
}
/*
#[bench]
fn search_speed(b: &mut test::Bencher) {
    use std::str::FromStr;
    use chess::ChessMove;

    let fen = "3q1rk1/5ppp/2n2n2/p1pNb3/3pP3/3P3N/PPbB2PP/R3KB1R b KQ - 1 16";

    let mut engine = Engine::new();
    engine.load_fen(fen).unwrap();

    b.iter(|| {
        for i in 0..4 {
            let search_data = engine.search_data.clone();
            engine.alpha_beta_search(i, search_data);
        }
    });
}*/