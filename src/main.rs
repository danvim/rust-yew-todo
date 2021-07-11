use yew::prelude::*;
use yew_functional::*;

use components::todo_app::TodoApp;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <TodoApp />
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::new(log::Level::Debug));
    yew::start_app::<App>();
}
