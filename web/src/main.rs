mod components;

use yew::prelude::*;
use components::select::Select;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <Select />
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
