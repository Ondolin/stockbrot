#![feature(test)]
extern crate test;

use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use chess::{ChessMove, Color, Game};

use opening_db::NODE_MAP;
use opening_db_types::Node as OpeningDBNode;

use crate::search::SearchData;

pub mod evaluation;
mod search;
mod transposition_table;

pub struct Engine {
    game: Game,
    search_data: Arc<SearchData>,
    opening_db_node: Option<OpeningDBNode>,
    moves_made: u16,
}

impl Engine {
    pub fn new() -> Engine {
        let e = Engine {
            game: Game::new(),
            search_data: Arc::new(SearchData::new()),
            opening_db_node: Some(NODE_MAP),
            moves_made: 0
        };

        e.search_data.visit_position(&e.game.current_position());

        e
    }

    pub fn get_position(&self) -> String {
        self.game.current_position().to_string()
    }

    pub fn load_fen(&mut self, fen: &str) -> Result<(), ()> {
        let game = Game::from_str(fen);
        self.opening_db_node = None;
        match game {
            Ok(game) => {
                self.game = game;
                self.search_data.visit_position(&self.game.current_position());
                Ok(())
            },
            Err(_) => Err(())
        }
    }

    pub fn make_move(&mut self, joice: String) {

        if let Some(opening_node) = &mut self.opening_db_node {
            self.opening_db_node = opening_node.get_node_by_move(joice.clone());
        }

        // here only my own moves are counted
        if self.opening_db_node.is_none() {
            self.moves_made += 1;
        }

        let joice = ChessMove::from_str(&joice).expect("No valid Chess move...");

        log::info!("Made move {:?}", joice.to_string());

        self.game.make_move(joice);

        self.search_data.visit_position(&self.game.current_position());
    }

    pub fn get_engine_move(&mut self, timeout: Duration) -> String {
        log::info!("Generating move...");

       if let Some(opening_book) = &self.opening_db_node {
            if let Some(opening_move) = opening_book.get_best_move() {
                log::info!("Opening DB move: {opening_move}");
                return opening_move;
            }
        }

        let joice = self.iterative_deepening(self.search_data.clone(), timeout).to_string();

        log::warn!("Engine Move: {joice}");
        log::warn!("Pos: {}", self.game.current_position().to_string());

        joice
    }

    pub fn is_my_turn(&self, color: Color) -> bool {
        self.game.side_to_move() == color
    }

    pub fn recommended_timeout(&self, time_remaining: Duration, increase: Duration) -> Duration {

        let default_timeout = Duration::from_secs(dotenv::var("DEFAULT_TIMEOUT_SECS").unwrap().parse::<u64>().unwrap());

        // Time = inf
        if time_remaining.as_secs() > 2_000_000 {
            return default_timeout;
        }

        let n_moves = self.moves_made.min(10);
        let factor = 2 - n_moves / 10;

        let potential_moves_left = 100 - self.moves_made.min(80);
        assert!(potential_moves_left >= 10);

        let target = time_remaining.as_millis() as u64 / potential_moves_left as u64;

        let recommended_timeout = Duration::from_millis(target * factor as u64) + increase;

        if recommended_timeout.as_secs() >= 5 {
            default_timeout
        } else {
            recommended_timeout
        }

    }

    pub fn do_off_move_stuff(&self) {

        self.search_data.transposition_table.age_table();
        self.search_data.previous_score.lock().unwrap().clear();

    }
}

