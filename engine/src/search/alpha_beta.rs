use std::fmt;
use std::sync::{Arc, RwLock};
use std::sync::atomic::Ordering;
use chess::{Board, BoardStatus, ChessMove, Color, MoveGen};
use crate::Engine;
use crate::search::quiesce_search::{quiesce_search_max, quiesce_search_min};
use crate::search::move_order::get_move_order;

use rayon::prelude::*;
use crate::evaluation::{CONSIDERED_MATE, MATE_SCORE};
use crate::search::{NodeType, SearchData, STOP_THREADS};

const WINDOW_SIZE: i32 = 100 / 4;
const DOUBLE_WINDOW_SIZE: i32 = WINDOW_SIZE * 2;
const QUADRUPLE_WINDOW_SIZE: i32 = WINDOW_SIZE * 4;

const MAX_WINDOW_SIZE: i32 = 2 * MATE_SCORE + 10;

struct AspirationWindow {
    source: i32,
    left: i32,
    right: i32,
}

impl AspirationWindow {
    pub fn new(source: i32) -> AspirationWindow {
        AspirationWindow {
            source,
            left: WINDOW_SIZE,
            right: WINDOW_SIZE,
        }
    }

    pub fn new_inf() -> AspirationWindow {
        AspirationWindow {
            source: 0,
            left: MAX_WINDOW_SIZE,
            right: MAX_WINDOW_SIZE
        }
    }

    pub fn next(current: i32) -> i32 {
        match current {
            WINDOW_SIZE => DOUBLE_WINDOW_SIZE,
            DOUBLE_WINDOW_SIZE => QUADRUPLE_WINDOW_SIZE,
            QUADRUPLE_WINDOW_SIZE => MAX_WINDOW_SIZE,
            e => panic!("{e} is not a valid window size to expand!")
        }
    }

    pub fn enlarge_and_check_bound(&mut self, value: i32) -> bool {
        if value <= self.alpha() {
            self.left = Self::next(self.left);
            false
        } else if value >= self.beta() {
            self.right = Self::next(self.right);
            false
        } else {
            true
        }
    }

    pub fn alpha(&self) -> i32 {
        self.source - self.left
    }

    pub fn beta(&self) -> i32 {
        self.source + self.right
    }
}

impl fmt::Display for AspirationWindow {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} | [{}, {}] => [{}, {}]", self.source, self.left, self.right, self.alpha(), self.beta())
    }
}

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

        if moves.len() == 1 { return (Some(moves[0]), 0) }

        moves.par_iter().for_each(|joice| {

            if STOP_THREADS.load(Ordering::SeqCst) { return; }

            let copy = self.game.current_position().make_move_new(*joice);
            let board_hash = copy.get_hash();

            // prevent repetition of moves
            if self.search_data.position_visited_twice(&copy) {
                log::info!("Can not play {} due to repetition of moves", joice.to_string());
                return;
            }

            let mut window = {
                if let Some(previous_move) = self.search_data.previous_score.lock().unwrap().get(&board_hash) {
                    AspirationWindow::new(*previous_move)
                } else {
                    AspirationWindow::new_inf()
                }
            };

            loop {

                if STOP_THREADS.load(Ordering::SeqCst) { return; }

                if self.game.side_to_move() == Color::White {

                    let (score, _) = alpha_beta_min(copy, window.alpha(), window.beta(), max_depth - 1, search_data.clone());

                    if !window.enlarge_and_check_bound(score) {
                        // window was to small
                        continue;
                    }

                    if STOP_THREADS.load(Ordering::SeqCst) { return; }

                    log::info!("Move Evaluation: {} {score}", joice.to_string());

                    { self.search_data.previous_score.lock().unwrap().insert(board_hash, score); }

                    if score >= { best_move.read().unwrap().1 } {
                        *best_move.write().unwrap() = (Some(*joice), score);
                    }
                } else {
                    let (score, _) = alpha_beta_max(copy, window.alpha(), window.beta(), max_depth - 1, search_data.clone());

                    if !window.enlarge_and_check_bound(score) {
                        // window was to small
                        continue;
                    }

                    if STOP_THREADS.load(Ordering::SeqCst) { return; }

                    log::info!("Move Evaluation: {} {score}", joice.to_string());

                    { self.search_data.previous_score.lock().unwrap().insert(board_hash, score); }

                    if score <= { best_move.read().unwrap().1 } {
                        *best_move.write().unwrap() = (Some(*joice), score);
                    }
                }

                break;
            }

        });

        let best_move = *best_move.read().unwrap();

        best_move
    }

}

// Mate in 1 should be a better score than mate in 2
fn consider_short_mate(value: i32) -> i32 {
    if value > CONSIDERED_MATE {
        value - 1
    } else if value < -CONSIDERED_MATE {
        value + 1
    } else {
        value
    }
}

pub fn alpha_beta_max(board: Board, mut alpha: i32, beta: i32, depth_left: u8, search_data: Arc<SearchData>) -> (i32, NodeType) {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_max(board, alpha, beta);
    }

    let mut best_move: Option<ChessMove> = None;

    for joice in get_move_order(&board, search_data.clone()) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);

        let score = SearchData::get_or_calculate(
            search_data.clone(),
            copy.get_hash(),
            alpha,
            beta,
            depth_left - 1,
            |data| alpha_beta_min(copy, alpha, beta, depth_left - 1, data)
        );

        // Score is outside of the window
        if score >= beta { return (beta, NodeType::CUT) }

        // Make window smaller
        if score > alpha {
            alpha = score;
            best_move = Some(joice);
        }

    }

    if let Some(best_move) = best_move { // node is PV
        search_data.best_moves.insert(board.get_hash(), best_move);
        (consider_short_mate(alpha), NodeType::PV)
    } else { // node is bound
        (alpha, NodeType::ALL)
    }

}

pub fn alpha_beta_min(board: Board, alpha: i32, mut beta: i32, depth_left: u8, search_data: Arc<SearchData>) -> (i32, NodeType) {
    // leaf node
    if depth_left == 0 || board.status() != BoardStatus::Ongoing {
        return quiesce_search_min(board, alpha, beta);
    }

    let mut best_move: Option<ChessMove> = None;

    for joice in get_move_order(&board, search_data.clone()) {

        if STOP_THREADS.load(Ordering::SeqCst) { break; }

        let copy = board.make_move_new(joice);

        let score = SearchData::get_or_calculate(
            search_data.clone(),
            copy.get_hash(),
            alpha,
            beta,
            depth_left - 1,
            |data| alpha_beta_max(copy, alpha, beta, depth_left - 1, data)
        );

        // Score is outside of the window
        if score <= alpha { return (alpha, NodeType::CUT) }

        // Make window smaller
        if score < beta {
            beta = score;
            best_move = Some(joice);
        }
    }

    if let Some(best_move) = best_move { // node is PV
        search_data.best_moves.insert(board.get_hash(), best_move);
        (consider_short_mate(beta), NodeType::PV)
    } else { // node is bound
        (beta, NodeType::ALL)
    }

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