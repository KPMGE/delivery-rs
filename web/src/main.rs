mod components;
// use ws::{connect, CloseCode};

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
    // let url = "ws://localhost:3012";
    // connect(url, |out| {
    //     println!("websocket server connected!");

    //     // out.send("{ \"routeId\": \"2\"}").unwrap();

    //     move |msg| {
    //         println!("Got message: {}", msg);
    //         out.close(CloseCode::Normal)
    //     }
    // }).unwrap();

    yew::Renderer::<App>::new().render();
}
