use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dotenv;
use env_logger::Env;

mod event_stream;
mod game;

use event_stream::StreamEvent;
use crate::game::listen_to_game;
use crate::event_stream::stream_game_updates;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let open_games: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    stream_game_updates(open_games.clone());

    loop {
        'l: loop {
            {
                if open_games.lock().unwrap().len() > 0 {
                    break 'l;
                }
            }
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        let current_game = {
            let open_games = open_games.lock().unwrap();
            open_games.first().unwrap().clone()
        };

        listen_to_game(current_game).await

    }

    // This loop keeps all threads alive
    loop {}

    Ok(())
}