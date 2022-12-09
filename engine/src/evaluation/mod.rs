mod evaluate;
mod good_piece_sq_tables;
mod simple_piece_sq_tables;

pub const MATE_SCORE: i32 = 1_000_000;

pub use evaluate::evaluate;
pub use good_piece_sq_tables::{mg_value, eg_value, game_phase_inc};