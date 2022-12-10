use chess::Color;
use serde::{Deserialize, Serialize};
use engine::Engine;

#[derive(Serialize, Deserialize, Debug)]
struct GameInfo {
    white: GamePlayer,
    black: GamePlayer,
    state: GameState,
    #[serde(rename = "initialFen")]
    initial_fen: Option<String>
}

impl GameInfo {
    fn my_color(&self) -> Color {
        if self.white.is_me() { Color::White }
        else if self.black.is_me() { Color::Black }
        else { panic!("Why the hell arent you part of that game?!") }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct GamePlayer {
    #[serde(rename = "aiLevel")]
    ai_level: Option<u8>,
    id: Option<String>,
}

impl GamePlayer {
    fn is_me(&self) -> bool {
        self.id == Some(dotenv::var("LICHESS_USERNAME").unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum GameState {
    #[serde(rename = "gameState")]
    StateEvent {
        moves: String,
        wtime: u32,
        btime: u32
    },
    #[serde(rename = "chatLine")]
    ChatEvent {},
    #[serde(rename = "opponentGone")]
    OpponentGone {}
}

pub async fn listen_to_game(game_id: String) {

    let mut engine = Engine::new();

    let client = reqwest::Client::new();
    let mut req = client.get(format!("https://lichess.org/api/bot/game/stream/{}", game_id))
        .bearer_auth(dotenv::var("LICHESS_TOKEN").unwrap())
        .send()
        .await
        .unwrap();

    if let Some(chunk) = req.chunk().await.unwrap() {

        let game_info: GameInfo = serde_json::from_slice(&chunk).unwrap();

        match game_info.state {
            GameState::StateEvent { ref moves, .. } => {

                if let Some(fen) = &game_info.initial_fen {
                    if fen != "startpos" {
                        engine.load_fen(fen).expect("Valid fen");
                    }
                }

                for joice in moves.split(' ') {
                    if joice != "" {
                        engine.make_move(joice.to_string());
                    }
                }

                log::info!("Loaded Game: {}", engine.get_position());

                if engine.is_my_turn(game_info.my_color()) {
                    post_move(&client, game_id.clone(), engine.get_engine_move()).await;
                }


            }
            _ => unreachable!()
        }

        // ignore that packet
        //let _ = req.chunk().await.unwrap();

        while let Some(chunk) = req.chunk().await.unwrap() {

            if chunk == "\n" { continue };

            let game_state: GameState = serde_json::from_slice(&chunk).unwrap();

            match game_state {
                GameState::StateEvent { moves, .. } => {

                    let moves: Vec<&str> = moves.split(' ').collect();

                    // update move in engine
                    engine.make_move(moves.last().unwrap().to_string());

                    // check if it is our move
                    if engine.is_my_turn(game_info.my_color()) {
                        post_move(&client, game_id.clone(), engine.get_engine_move()).await;
                    }

                },
                _ => {}
            }

        }

    }
}

async fn post_move(client: &reqwest::Client, game_id: String, engine_move: String) {
    // post move
    client.post(format!("https://lichess.org/api/bot/game/{}/move/{}", game_id, engine_move))
        .bearer_auth(dotenv::var("LICHESS_TOKEN").unwrap())
        .send()
        .await
        .unwrap();
}