#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use std::sync::atomic::AtomicBool;
use std::time::Duration;
use chashmap::CHashMap;
use chess::{Board, ChessMove, Color, Error, Game, MoveGen};

use crate::evaluate::evaluate;
use crate::evaluated_position::EvaluatedPositions;

mod evaluate;
mod search;
mod quiesce_search;
mod evaluated_position;

pub struct Engine {
    game: Game,
    evaluated_positions: Arc<RwLock<EvaluatedPositions>>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            game: Game::new(),
            evaluated_positions: Arc::new(RwLock::new(CHashMap::new()))
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
            Err(e) => Err(())
        }
    }

    pub fn make_move(&mut self, joice: String) {
        let joice = ChessMove::from_str(&joice).expect("No valid Chess move...");

        log::info!("Made move {:?}", joice.to_string());

        self.game.make_move(joice);
    }

    pub fn get_engine_move(&mut self) -> String {
        log::info!("Generating move...");
        let joice = self.iterative_deepening(self.evaluated_positions.clone(), Duration::from_secs(8)).to_string();

        log::warn!("Engine Move: {joice}");
        log::warn!("Pos: {}", self.game.current_position().to_string());

        joice
    }

    pub fn is_my_turn(&self, color: Color) -> bool {
        self.game.side_to_move() == color
    }
}

