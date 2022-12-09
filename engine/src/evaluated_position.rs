use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chashmap::CHashMap;
use chess::Board;
use crate::evaluate::MATE_SCORE;
use crate::search::CURRENT_SEARCH_DEPTH;

#[derive(Clone)]
pub struct EvaluatedPosition {
    pub board: Board,
    pub evaluation: i32,
    pub depth: u8
}

impl PartialEq<Self> for EvaluatedPosition {
    fn eq(&self, other: &Self) -> bool {
        self.depth == other.depth
    }
}

impl PartialOrd<Self> for EvaluatedPosition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.depth.cmp(&other.depth))
    }
}

pub type EvaluatedPositions = CHashMap<u64, EvaluatedPosition>;

pub trait EvaluatedPositionsFunctions {
    fn get_or_calculate<F>(&self, board: Board, depth: u8, calculate: F) -> i32 where F: Fn(Arc<RwLock<EvaluatedPositions>>) -> i32;
}

impl EvaluatedPositionsFunctions for Arc<RwLock<EvaluatedPositions>> {

    fn get_or_calculate<F>(&self, board: Board, depth: u8, mut calculate: F) -> i32
        where F: Fn(Arc<RwLock<EvaluatedPositions>>) -> i32 {

        let hash = board.get_hash();

        if let Some(old) = { self.read().unwrap().get(&hash) } {
            // we have a better value in the database
            if old.depth >= depth {
                return old.evaluation;
            }
        }

        let mut evaluation = calculate(self.clone());

        let max_search_depth = CURRENT_SEARCH_DEPTH.load(core::sync::atomic::Ordering::Relaxed) as i32;

        // shorter mate has high score
        if evaluation == MATE_SCORE {
            evaluation -= max_search_depth - depth as i32;
        } else if evaluation == -MATE_SCORE {
            evaluation += max_search_depth - depth as i32;
        }

        self.read().unwrap().insert(hash, EvaluatedPosition {
            board,
            evaluation,
            depth
        });

        evaluation

    }
}