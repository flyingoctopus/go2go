// client/src/components.rs
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct ButtonProps {
    pub label: String,
    pub onclick: Callback<MouseEvent>, // Specify the event type
}

#[function_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()}>{ &props.label }</button>
    }
}
