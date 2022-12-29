use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, AtomicU8};
use std::sync::{Arc, Mutex, RwLock};
use chashmap::CHashMap;
use chess::{Board, ChessMove};
use crate::transposition_table::entry::Entry;
use crate::transposition_table::table::TranspositionTable;

mod move_order;
mod alpha_beta;
mod iterative_deepening;
mod quiesce_search;

static STOP_THREADS: AtomicBool = AtomicBool::new(false);
pub static CURRENT_SEARCH_DEPTH: AtomicU8 = AtomicU8::new(0);

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum NodeType {
    PV, // Exact value
    CUT, // Lower bound
    ALL // Upper bound
}

pub struct SearchData {
    pub transposition_table: TranspositionTable,
    pub previous_score: Mutex<HashMap<u64, i32>>,
    positions_visited: RwLock<HashMap<u64, u8>>,
    best_moves: CHashMap<u64, ChessMove>
}

impl SearchData {
    pub fn new() -> SearchData {
        SearchData {
            transposition_table: TranspositionTable::new(),
            previous_score: Mutex::new(HashMap::new()),
            positions_visited: RwLock::new(HashMap::new()),
            best_moves: CHashMap::new()
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

    pub fn get_or_calculate<F>(this: Arc<SearchData>, hash: u64, alpha: i32, beta: i32, depth: u8, calculate: F) -> i32
        where F: Fn(Arc<SearchData>) -> (i32, NodeType) {

        let entry = this.transposition_table.get(hash);

        // check if entry has been calculated
        'stored_value: {
            let entry = entry.read().unwrap();

            if let Entry::Contains { depth: _depth, hash: _hash, score, node_type, .. } = *entry {

                // check if the stored bound is good enough
                match node_type {
                    NodeType::CUT => if beta > score { break 'stored_value } /* bad score */
                    NodeType::ALL => if alpha < score { break 'stored_value } /* bad score */
                    NodeType::PV => { if !(alpha <= score && score <= beta) { break 'stored_value } } // TODO think about this
                }

                // value has been searched deep enough
                if hash == _hash && _depth >= depth {
                    return score
                }
            }

        }

        let (score, node_type) = calculate(this.clone());

        // push score to transposition table
        {
            let mut entry = entry.write().unwrap();
            *entry = Entry::Contains {
                hash,
                depth,
                score,
                node_type,
                age: 0,
            }
        }

        score

    }
}