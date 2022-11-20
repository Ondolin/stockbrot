use std::collections::HashMap;
use std::str::FromStr;
use std::sync::{Arc, RwLock};
use chess::{Board, ChessMove, Game, MoveGen};

use crate::evaluate::evaluate;
use crate::evaluated_position::EvaluatedPositions;

mod evaluate;
mod search;
mod evaluated_position;

pub struct Engine {
    game: Game,
    evaluated_positions: Arc<RwLock<EvaluatedPositions>>
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            game: Game::new(),
            evaluated_positions: Arc::new(RwLock::new(HashMap::new()))
        }
    }

    pub fn make_move(&mut self, joice: String) {
        let joice = ChessMove::from_str(&joice).expect("No valid Chess move...");

        log::info!("Made move {:?}", joice.to_string());

        self.game.make_move(joice);
    }

    pub fn get_engine_move(&mut self) -> String {
        let joice = self.alpha_beta_search(10, self.evaluated_positions.clone()).to_string();
        log::warn!("Engine Move: {joice}");
        joice
    }
}

