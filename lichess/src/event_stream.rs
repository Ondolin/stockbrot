use std::env;
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type", content = "challenge")]
pub enum ChallengeEvent {
    #[serde(rename = "challenge")]
    Challenge {
        id: String,
        challenger: ChallengeUser
    },
    #[serde(rename = "challengeCanceled")]
    ChallengeCanceled {
        id: String
    },
    #[serde(rename = "challengeDeclined")]
    ChallengeDeclined {
        id: String
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChallengeUser {
    id: String
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

            let event: serde_json::error::Result<StreamEvent> = serde_json::from_slice(&chunk);
            if let Ok(event) = event {
                match event {
                    StreamEvent::GameStart { game_id } => { games.lock().unwrap().push(game_id) }
                    StreamEvent::GameFinish { game_id } => {
                        let mut games = games.lock().unwrap();
                        let index = games.iter().position(|x| *x == game_id).unwrap();
                        games.remove(index);
                    }
                }
            }

            let event: serde_json::error::Result<ChallengeEvent> = serde_json::from_slice(&chunk);
            if let Ok(ChallengeEvent::Challenge { ref id, ref challenger }) = event {

                // ignore challenges started by us
                if challenger.id != dotenv::var("LICHESS_USERNAME").unwrap() {

                    log::info!("Accepted Challenge with ID: \"{}\" against \"{}\"", id, challenger.id);

                    client.post(format!("https://lichess.org/api/challenge/{}/accept", id))
                        .bearer_auth(dotenv::var("LICHESS_TOKEN").unwrap())
                        .send()
                        .await
                        .unwrap();
                }
            }

        }
    });
}