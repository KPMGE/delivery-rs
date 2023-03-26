mod components;
mod models;

use yew::prelude::*;
use components::select::Select;

#[function_component]
fn App() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <div>
            <Select />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
