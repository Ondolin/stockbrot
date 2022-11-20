use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chess::{Board, BoardStatus, ChessMove, Color, EMPTY, MoveGen};
use crate::{Engine, evaluate, EvaluatedPositions};

use rayon::prelude::*;
use crate::evaluated_position::EvaluatedPositionsFunctions;

impl Engine {
    pub fn alpha_beta_search(&self, max_depth: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> ChessMove {
        let moves = MoveGen::new_legal(&self.game.current_position());

        let mut best_move: RwLock<Option<ChessMove>> = RwLock::new(None);
        let mut best_score = RwLock::new(if self.game.side_to_move() == Color::White { f32::NEG_INFINITY } else { f32::INFINITY });

        moves.collect::<Vec<ChessMove>>().par_iter().for_each(|joice| {
            let copy = self.game.current_position().make_move_new(*joice);

            let alpha = f32::NEG_INFINITY;
            let beta = f32::INFINITY;

            if self.game.side_to_move() == Color::White {
                let score = alpha_beta_min(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());

                if score >= { *best_score.read().unwrap() } {
                    *best_move.write().unwrap() = Some(*joice);
                    *best_score.write().unwrap() = score;
                }
            } else {
                let score = alpha_beta_max(copy, alpha, beta, max_depth - 1, evaluated_positions.clone());

                if score <= { *best_score.read().unwrap() } {
                    *best_move.write().unwrap() = Some(*joice);
                    *best_score.write().unwrap() = score;
                }
            }

        });

        let x = best_move.read().unwrap().clone().expect("Best move."); x
    }
}

pub fn alpha_beta_max(board: Board, mut alpha: f32, beta: f32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> f32 {
// pub fn alpha_beta_max(board: Board, mut alpha: f32, beta: f32, depth_left: u8) -> f32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluate(&board) }

    let mut moves = MoveGen::new_legal(&board);

    let mut value = f32::NEG_INFINITY;

    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);
    for joice in &mut moves {
        let copy = board.make_move_new(joice);

        // value = value.max(alpha_beta_min(copy, alpha, beta, depth_left - 1));
        value = value.max(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| alpha_beta_min(copy, alpha, beta, depth_left - 1, eval)));

        if value >= beta { break }

        alpha = alpha.max(value);

    }
    moves.set_iterator_mask(!EMPTY);
    for joice in &mut moves {
        let copy = board.make_move_new(joice);

        // value = value.max(alpha_beta_min(copy, alpha, beta, depth_left - 1));
        value = value.max(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| alpha_beta_min(copy, alpha, beta, depth_left - 1, eval)));

        if value >= beta { break }

        alpha = alpha.max(value);

    }

    value
}

pub fn alpha_beta_min(board: Board, alpha: f32, mut beta: f32, depth_left: u8, evaluated_positions: Arc<RwLock<EvaluatedPositions>>) -> f32 {
// pub fn alpha_beta_min(board: Board, alpha: f32, mut beta: f32, depth_left: u8) -> f32 {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing { return evaluate(&board) }

    let mut moves = MoveGen::new_legal(&board);

    let mut value = f32::INFINITY;

    let targets = board.color_combined(!board.side_to_move());
    moves.set_iterator_mask(*targets);
    for joice in &mut moves {
        let copy = board.make_move_new(joice);

        // value = value.min(alpha_beta_max(copy, alpha, beta, depth_left - 1));
        value = value.max(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));

        if value <= alpha { break }

        beta = beta.min(value);
    }
    moves.set_iterator_mask(!EMPTY);
    for joice in &mut moves {
        let copy = board.make_move_new(joice);

        // value = value.min(alpha_beta_max(copy, alpha, beta, depth_left - 1));
        value = value.max(evaluated_positions
            .get_or_calculate(
                copy,
                depth_left,
                |eval| alpha_beta_max(copy, alpha, beta, depth_left - 1, eval)));


    if value <= alpha { break }

    beta = beta.min(value);
}

value
}