use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::RwLock;
use chashmap::CHashMap;
use chess::Board;
use crate::search::evaluated_position::EvaluatedPositions;

mod move_order;
mod alpha_beta;
mod iterative_deepening;
mod quiesce_search;
pub mod evaluated_position;

static STOP_THREADS: AtomicBool = AtomicBool::new(false);
pub static CURRENT_SEARCH_DEPTH: AtomicU8 = AtomicU8::new(0);

pub struct SearchData {
    evaluated_positions: EvaluatedPositions,
    positions_visited: RwLock<HashMap<u64, u8>>
}

impl SearchData {
    pub fn new() -> SearchData {
        SearchData {
            evaluated_positions: CHashMap::new(),
            positions_visited: RwLock::new(HashMap::new())
        }
    }

    pub fn position_visited_twice(&self, position: &Board) -> bool {
        if let Some(visited) = self.positions_visited.read().unwrap().get(&position.get_hash()) {
            return visited >= &2
        }

        false
    }

    pub fn visit_position(&self, position: &Board) {
        let hash = position.get_hash();
        let mut visited_lock = self.positions_visited.write().unwrap();

        let mut visited = 0;

        if let Some(&v) = visited_lock.get(&hash) {
            visited = v;
        }

        visited += 1;

        visited_lock.insert(hash, visited);
    }
}