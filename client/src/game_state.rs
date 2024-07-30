// client/src/game_state.rs
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GameState {
    pub board: Vec<Vec<Option<Player>>>,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub enum Player {
    Black,
    White,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            board: vec![vec![None; 19]; 19],
        }
    }

    // Add methods to manage game state, like update, reset, etc.
}
