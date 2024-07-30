// client/src/lib.rs
use yew::prelude::*;
use wasm_bindgen::prelude::*;

mod components;
mod game_state;

#[wasm_bindgen(start)]
pub fn run_app() {
    yew::start_app::<Model>();
}

pub struct Model {
    game_state: game_state::GameState,
}

pub enum Msg {
    Clicked(usize, usize),
    UpdateState(game_state::GameState),
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let game_state = game_state::GameState::new();
        Self { game_state }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Clicked(x, y) => {
                // Handle click events here
                // Example: self.game_state.update(x, y);
            }
            Msg::UpdateState(new_state) => {
                self.game_state = new_state;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <h1>{ "Go Game" }</h1>
                <div class="board">
                    { for (0..19).map(|x| self.view_row(ctx, x)) }
                </div>
            </div>
        }
    }
}

impl Model {
    fn view_row(&self, ctx: &Context<Self>, x: usize) -> Html {
        html! {
            <div class="row">
                { for (0..19).map(|y| self.view_cell(ctx, x, y)) }
            </div>
        }
    }

    fn view_cell(&self, ctx: &Context<Self>, x: usize, y: usize) -> Html {
        let content = match self.game_state.board[x][y] {
            Some(game_state::Player::Black) => "●",
            Some(game_state::Player::White) => "○",
            None => " ",
        };
        let onclick = ctx.link().callback(move |_| Msg::Clicked(x, y));
        html! {
            <button {onclick}>{ content }</button>
        }
    }
}
