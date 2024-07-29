use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Serialize, Deserialize, Clone)]
struct GameState {
    board: Vec<Vec<Option<Player>>>,
    turn: Player,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
enum Player {
    Black,
    White,
}

#[tokio::main]
async fn main() {
    let game_state = GameState {
        board: vec![vec![None; 19]; 19],
        turn: Player::Black,
    };

    let game_state = warp::any().map(move || warp::reply::json(&game_state));

    warp::serve(game_state).run(([127, 0, 0, 1], 3030)).await;
}
