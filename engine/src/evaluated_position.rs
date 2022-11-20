use std::cmp::Ordering;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use chess::Board;

#[derive(Clone)]
pub struct EvaluatedPosition {
    pub board: Board,
    pub evaluation: f32,
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

pub type EvaluatedPositions = HashMap<u64, EvaluatedPosition>;

pub trait EvaluatedPositionsFunctions {
    fn get_or_calculate<F>(&self, board: Board, depth: u8, calculate: F) -> f32 where F: Fn(Arc<RwLock<EvaluatedPositions>>) -> f32;
}

impl EvaluatedPositionsFunctions for Arc<RwLock<EvaluatedPositions>> {

    fn get_or_calculate<F>(&self, board: Board, depth: u8, mut calculate: F) -> f32
        where F: Fn(Arc<RwLock<EvaluatedPositions>>) -> f32{

        let hash = board.get_hash();

        if let Some(old) = { self.read().unwrap().get(&hash).clone() } {
            // we have a better value in the database
            if old.depth >= depth {
                if depth > 4 {
                    log::error!("Saved: {depth}");
                }

                return old.evaluation;
            }
        }

        let evaluation = calculate(self.clone());

        self.write().unwrap().insert(hash, EvaluatedPosition {
            board,
            evaluation,
            depth
        });

        evaluation

    }
}