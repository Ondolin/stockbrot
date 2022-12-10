#![feature(test)]
extern crate test;

use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use chashmap::CHashMap;
use chess::{ChessMove, Color, Game};

use opening_db::NODE_MAP;
use opening_db_types::Node as OpeningDBNode;

use crate::evaluated_position::EvaluatedPositions;

pub mod evaluation;
mod search;
mod quiesce_search;
mod evaluated_position;

pub struct Engine {
    game: Game,
    evaluated_positions: Arc<RwLock<EvaluatedPositions>>,
    opening_db_node: Option<OpeningDBNode>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            game: Game::new(),
            evaluated_positions: Arc::new(RwLock::new(CHashMap::new())),
            opening_db_node: Some(NODE_MAP)
        }
    }

    pub fn get_position(&self) -> String {
        self.game.current_position().to_string()
    }

    pub fn load_fen(&mut self, fen: &str) -> Result<(), ()> {
        let game = Game::from_str(fen);
        match game {
            Ok(game) => {
                self.game = game;
                Ok(())
            },
            Err(_) => Err(())
        }
    }

    pub fn make_move(&mut self, joice: String) {

        if let Some(opening_node) = &mut self.opening_db_node {
            self.opening_db_node = opening_node.get_node_by_move(joice.clone());
        }

        let joice = ChessMove::from_str(&joice).expect("No valid Chess move...");

        log::info!("Made move {:?}", joice.to_string());

        self.game.make_move(joice);
    }

    pub fn get_engine_move(&mut self) -> String {
        log::info!("Generating move...");

        if let Some(opening_book) = &self.opening_db_node {
            if let Some(opening_move) = opening_book.get_best_move() {
                log::info!("Opening DB move: {opening_move}");
                return opening_move;
            }
        }

        let joice = self.iterative_deepening(self.evaluated_positions.clone(), Duration::from_secs(8)).to_string();

        log::warn!("Engine Move: {joice}");
        log::warn!("Pos: {}", self.game.current_position().to_string());

        joice
    }

    pub fn is_my_turn(&self, color: Color) -> bool {
        self.game.side_to_move() == color
    }
}

