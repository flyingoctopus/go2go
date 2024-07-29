use yew::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq)]
struct GameState {
    board: Vec<Vec<Option<Player>>>,
    turn: Player,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq)]
enum Player {
    Black,
    White,
}

enum Msg {
    Move(usize, usize),
    UpdateState(GameState),
}

struct Model {
    link: ComponentLink<Self>,
    state: GameState,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = GameState {
            board: vec![vec![None; 19]; 19],
            turn: Player::Black,
        };
        Self { link, state }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Move(x, y) => {
                // Handle move logic here
            }
            Msg::UpdateState(new_state) => {
                self.state = new_state;
            }
        }
        true
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <h1>{ "Go Game" }</h1>
                <div>
                    { for (0..19).map(|x| self.view_row(x)) }
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_row(&self, x: usize) -> Html {
        html! {
            <div>
                { for (0..19).map(|y| self.view_cell(x, y)) }
            </div>
        }
    }

    fn view_cell(&self, x: usize, y: usize) -> Html {
        let onclick = self.link.callback(move |_| Msg::Move(x, y));
        let content = match self.state.board[x][y] {
            Some(Player::Black) => "●",
            Some(Player::White) => "○",
            None => " ",
        };
        html! {
            <button onclick=onclick>{ content }</button>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
