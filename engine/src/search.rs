use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, AtomicU8, Ordering};
use std::time::Duration;
use chess::{Board, BoardStatus, ChessMove, Color, EMPTY, MoveGen};
use crate::{Engine, EvaluatedPositions};

use rayon::prelude::*;
use crate::evaluation::MATE_SCORE;
use crate::evaluated_position::EvaluatedPositionsFunctions;
use crate::quiesce_search::{quiesce_search_max, quiesce_search_min};

static STOP_THREADS: AtomicBool = AtomicBool::new(false);
pub static CURRENT_SEARCH_DEPTH: AtomicU8 = AtomicU8::new(0);

impl Engine {
    pub fn alpha_beta_search(&self, max_depth: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> (Option<ChessMove>, i32) {

        let best_move: RwLock<(Option<ChessMove>, i32)> =
            RwLock::new((
                None,
                if self.game.side_to_move() == Color::White { i32::MIN } else { i32::MAX }
            ));

        let moves = MoveGen::new_legal(&self.game.current_position()).collect::<Vec<ChessMove>>();
        moves.par_iter().for_each(|joice| {

            if STOP_THREADS.load(Ordering::SeqCst) { return; }

            let copy = self.game.current_position().make_move_new(*joice);

            let alpha = i32::MIN;
            let beta = i32::MAX;

            if self.game.side_to_move() == Color::White {

                let score = alpha_beta_min(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());
                if STOP_THREADS.load(Ordering::SeqCst) { return; }

                log::error!("up {} {score}", joice.to_string());

                if score >= { best_move.read().unwrap().1 } {
                    *best_move.write().unwrap() = (Some(*joice), score);
                }
            } else {
                let score = alpha_beta_max(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());
                if STOP_THREADS.load(Ordering::SeqCst) { return; }

                log::error!("down {} {score}", joice.to_string());

                if score <= { best_move.read().unwrap().1 } {
                    *best_move.write().unwrap() = (Some(*joice), score);
                }
            }

        });

        let best_move = *best_move.read().unwrap();

        best_move
    }

    pub fn iterative_deepening(&self, evaluated_positions: Arc<RwLock<EvaluatedPositions>>, timeout: Duration) -> ChessMove {

        let soft_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let soft_stop_copy = soft_stop.clone();

        STOP_THREADS.store(false, Ordering::SeqCst);
        CURRENT_SEARCH_DEPTH.store(0, Ordering::Relaxed);

        // Stop the search if time if over
        std::thread::spawn(move || {
            std::thread::park_timeout(timeout);

            soft_stop_copy.store(true, Ordering::SeqCst);

            // if at least depth 6 is searched hard stop
            if CURRENT_SEARCH_DEPTH.load(Ordering::Relaxed) > 4 {
                STOP_THREADS.store(true, Ordering::SeqCst);
            }


        });

        let mut best_move: Option<ChessMove> = None;
        for current_depth in (4..255).step_by(2) {
            CURRENT_SEARCH_DEPTH.store(current_depth, Ordering::Relaxed);

            let (new_best_move, score) = self.alpha_beta_search(current_depth, evaluated_positions.clone());

            if let Some(joice) = new_best_move {
                assert!(self.game.current_position().legal(joice));
            }

            // The current layer has been stopped before the calculation finished
            let hard_stop = STOP_THREADS.load(Ordering::SeqCst);

            // Time is up, but calculation has not been representative
            let soft_stop = soft_stop.load(Ordering::SeqCst);

            if hard_stop || new_best_move.is_none() {
                break;
            }

            log::info!("Best move of depth {current_depth} is {} with score: {score}", new_best_move.unwrap().to_string());

            best_move = new_best_move;

            if score.abs() > MATE_SCORE - 1_000 {
                break;
            }

            if current_depth >= 4 && soft_stop {
                break;
            }
        }

        best_move.expect("Could not find a good move...")

    }

}

// put captures before other moves
pub fn get_move_order(board: &Board) -> Vec<ChessMove> {
    let mut moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);

    let mut moves_in_order = moves.by_ref().collect::<Vec<ChessMove>>();
    moves.set_iterator_mask(!EMPTY);
    let mut second_half = moves.collect::<Vec<ChessMove>>();
    moves_in_order.append(&mut second_half);

    moves_in_order
}

pub fn alpha_beta_max(board: Board, mut alpha: i32, beta: i32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> i32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| {
        quiesce_search_max(board, i32::MIN, i32::MAX)
    } ) }

    let mut value = i32::MIN;

    for joice in get_move_order(&board) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);

        let score = evaluated_positions
            .get_or_calculate(
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

pub fn alpha_beta_min(board: Board, alpha: i32, mut beta: i32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> i32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| {
        quiesce_search_min(board, i32::MIN, i32::MAX)
    } ) }

    let mut value = i32::MAX;

    for joice in get_move_order(&board) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);
        value = value.min(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));

        if value <= alpha { break }

        beta = beta.min(value);
    }

    value
}