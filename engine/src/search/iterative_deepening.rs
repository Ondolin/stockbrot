use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use chess::ChessMove;
use crate::Engine;
use crate::search::evaluated_position::EvaluatedPositions;
use crate::evaluation::MATE_SCORE;
use crate::search::{CURRENT_SEARCH_DEPTH, SearchData, STOP_THREADS};

impl Engine {
    pub fn iterative_deepening(&self, search_data: Arc<SearchData>, timeout: Duration) -> ChessMove {

        let soft_stop: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let soft_stop_copy = soft_stop.clone();

        STOP_THREADS.store(false, Ordering::SeqCst);
        CURRENT_SEARCH_DEPTH.store(0, Ordering::Relaxed);

        // Stop the search if time if over
        std::thread::spawn(move || {
            std::thread::park_timeout(timeout);

            soft_stop_copy.store(true, Ordering::SeqCst);

            // if at least depth 6 is searched hard stop
            if CURRENT_SEARCH_DEPTH.load(Ordering::Relaxed) > 1 {
                STOP_THREADS.store(true, Ordering::SeqCst);
            }


        });

        let start_time = std::time::Instant::now();

        let mut best_move: Option<ChessMove> = None;
        for current_depth in (1..255).step_by(1) {
            CURRENT_SEARCH_DEPTH.store(current_depth, Ordering::Relaxed);

            let (new_best_move, score) = self.alpha_beta_search(current_depth, search_data.clone());

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

            if soft_stop {
                break;
            }

            log::error!("Time elaps: {}", start_time.elapsed().as_micros());
        }

        best_move.expect("Could not find a good move...")

    }
}