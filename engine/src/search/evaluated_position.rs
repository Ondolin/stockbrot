use std::cmp::Ordering;
use std::sync::Arc;
use chashmap::CHashMap;
use chess::Board;
use crate::search::SearchData;

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
    fn get_or_calculate_evaluation<F>(&self, board: Board, depth: u8, calculate: F) -> i32 where F: Fn(Arc<SearchData>) -> i32;
}

impl EvaluatedPositionsFunctions for Arc<SearchData> {

    fn get_or_calculate_evaluation<F>(&self, board: Board, depth: u8, calculate: F) -> i32
        where F: Fn(Arc<SearchData>) -> i32 {

        let hash = board.get_hash();

        if let Some(old) = self.evaluated_positions.get(&hash) {
            // we have a better value in the database
            if old.depth >= depth {
                return old.evaluation;
            }
        }

        let evaluation = calculate(self.clone());

        self.evaluated_positions.insert(hash, EvaluatedPosition {
            board,
            evaluation,
            depth
        });

        evaluation

    }
}