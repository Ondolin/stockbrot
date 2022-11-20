use std::sync::{Arc, Mutex};
use std::time::Duration;
use log::{error, log};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "game")]
pub enum StreamEvent {
    #[serde(rename = "gameStart")]
    GameStart {
        #[serde(rename = "gameId")]
        game_id: String
    },
    #[serde(rename = "gameFinish")]
    GameFinish {
        #[serde(rename = "gameId")]
        game_id: String
    }

}

pub fn stream_game_updates(games: Arc<Mutex<Vec<String>>>) {
    let _handle = tokio::spawn(async move {
        let lichess_key: String = dotenv::var("LICHESS_TOKEN").unwrap();

        let client = reqwest::Client::new();
        let mut req = client.get("https://lichess.org/api/stream/event")
            .bearer_auth(lichess_key)
            .send()
            .await
            .unwrap();

        while let Some(chunk) = req.chunk().await.unwrap() {
            if chunk == "\n" { continue };

            let event: StreamEvent = serde_json::from_slice(&chunk).unwrap();

            match event {
                StreamEvent::GameStart { game_id } => { games.lock().unwrap().push(game_id) }
                StreamEvent::GameFinish { game_id } => {
                    let mut games = games.lock().unwrap();
                    let index = games.iter().position(|x| *x == game_id).unwrap();
                    games.remove(index);
                }
            }

        }
    });
}