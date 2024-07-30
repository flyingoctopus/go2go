// server/src/main.rs
use serde::{Deserialize, Serialize};
use warp::Filter;

mod game_logic;

#[derive(Serialize, Deserialize, Clone, Debug)]
struct GameState {
    board: Vec<Vec<Option<Player>>>,
    turn: Player,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum Player {
    Black,
    White,
}

#[tokio::main]
async fn main() {
    let game_state = warp::path("game_state")
        .map(|| {
            let game = GameState {
                board: vec![vec![None; 19]; 19],
                turn: Player::Black,
            };
            warp::reply::json(&game)
        });

    warp::serve(game_state).run(([127, 0, 0, 1], 3030)).await;
}
