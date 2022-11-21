use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use atomic_float::AtomicF32;
use chess::{Board, BoardStatus, ChessMove, Color, EMPTY, MoveGen};
use crate::{Engine, evaluate, EvaluatedPositions};

use rayon::prelude::*;
use crate::evaluated_position::EvaluatedPositionsFunctions;

// const STOP_THREADS: AtomicBool = AtomicBool::new(false);

impl Engine {
    pub fn alpha_beta_search(&self, max_depth: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> ChessMove {

        let mut best_move: RwLock<Option<ChessMove>> = RwLock::new(None);
        let mut best_score = RwLock::new(if self.game.side_to_move() == Color::White { f32::NEG_INFINITY } else { f32::INFINITY });

        let moves = MoveGen::new_legal(&self.game.current_position()).collect::<Vec<ChessMove>>();
        moves.par_iter().for_each(|joice| {

            // if STOP_THREADS.load(Ordering::Relaxed) { return; }

            let copy = self.game.current_position().make_move_new(*joice);

            let alpha = f32::NEG_INFINITY;
            let beta = f32::INFINITY;

            if self.game.side_to_move() == Color::White {
                let score = alpha_beta_min(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());

                /*if score > 10_000.0 {
                    log::error!("Fin");
                    STOP_THREADS.store(true, Ordering::Relaxed);
                }*/

                log::error!("up {} {score}", joice.to_string());

                if score >= { *best_score.read().unwrap() } {
                    *best_move.write().unwrap() = Some(*joice);
                    *best_score.write().unwrap() = score;
                }
            } else {
                let score = alpha_beta_max(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());

                /*if score < -10_000.0 {
                    log::error!("Fin");
                    STOP_THREADS.store(true, Ordering::Relaxed);
                }*/

                log::error!("down {} {score}", joice.to_string());

                if score <= { *best_score.read().unwrap() } {
                    *best_move.write().unwrap() = Some(*joice);
                    *best_score.write().unwrap() = score;
                }
            }

        });

        let x = best_move.read().unwrap().clone().expect("Best move.");
        let y = best_score.read().expect("Best move.");

        log::warn!("Score: {}", y);
        x

    }
}

pub fn alpha_beta_max(board: Board, mut alpha: f32, beta: f32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> f32 {
// pub fn alpha_beta_max(board: Board, mut alpha: f32, beta: f32, depth_left: u8) -> f32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| evaluate(&board) ) }

    let value = AtomicF32::new(f32::NEG_INFINITY);

    let alpha = AtomicF32::new(alpha);
    let beta = AtomicF32::new(beta);

    let mut moves = MoveGen::new_legal(&board);
    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);

    let mut moves_in_order = moves.by_ref().collect::<Vec<ChessMove>>();
    moves.set_iterator_mask(!EMPTY);
    let mut second_half = moves.collect::<Vec<ChessMove>>();
    moves_in_order.append(&mut second_half);

    moves_in_order.into_par_iter().find_any(|joice| {

        // if STOP_THREADS.load(Ordering::Relaxed) { return true; }

        let copy = board.make_move_new(*joice);

        // value = value.max(alpha_beta_min(copy, alpha, beta, depth_left - 1));


        value.fetch_max(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| {
                    alpha_beta_min(
                        copy,
                        alpha.load(Ordering::Relaxed),
                        alpha.load(Ordering::Relaxed),
                        depth_left - 1,
                        eval)
                }
            ), Ordering::Relaxed);

        let value = value.load(Ordering::Relaxed);

        if value >= beta.load(Ordering::Relaxed) {
            return true;
        }

        alpha.fetch_max(value, Ordering::Relaxed);

        return false;
    });

    value.load(Ordering::Relaxed)
}

pub fn alpha_beta_min(board: Board, alpha: f32, mut beta: f32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> f32 {
// pub fn alpha_beta_min(board: Board, alpha: f32, mut beta: f32, depth_left: u8) -> f32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluated_positions.get_or_calculate(board, depth_left, |_| evaluate(&board) ) }

    let mut moves = MoveGen::new_legal(&board);

    let mut value = f32::INFINITY;

    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);
    'a: {
        for joice in &mut moves {

            // if STOP_THREADS.load(Ordering::Relaxed) { break; }

            let copy = board.make_move_new(joice);

            // value = value.min(alpha_beta_max(copy, alpha, beta, depth_left - 1));
            value = value.min(evaluated_positions
                .get_or_calculate(
                    copy,
                    depth_left,
                    |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));

            if value <= alpha { break 'a }

            beta = beta.min(value);
        }
        moves.set_iterator_mask(!EMPTY);
        for joice in &mut moves {

            // if STOP_THREADS.load(Ordering::Relaxed) { break; }

            let copy = board.make_move_new(joice);

            // value = value.min(alpha_beta_max(copy, alpha, beta, depth_left - 1));
            value = value.min(evaluated_positions
                .get_or_calculate(
                    copy,
                    depth_left,
                    |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));


            if value <= alpha { break 'a }

            beta = beta.min(value);
        }
    }

    value
}