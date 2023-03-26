use log::info;
// use wasm_bindgen::prelude::Closure;
use web_sys::{Event, HtmlSelectElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

#[function_component]
pub fn Select() -> Html {
    let ws = use_websocket("ws://localhost:3012".to_string());
    let selected_value = use_state::<Option<i32>, _>(|| None);

    let handle_onchange = {
        let selected_value = selected_value.clone();

        move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                let value = target.value().parse::<i32>().unwrap();
                selected_value.set(Some(value));
                info!("selected value: {}", value);
            }
        }
    };


    let request_route = {
        let selected_value = selected_value.clone();
        let ws = ws.clone();

        Callback::from(move |_| {
            match &*selected_value {
                Some(route_id) => {
                    // request the route
                    info!("requesting route...");
                    let message = format!("new-route {{ \"routeId\": \"{}\" }}", route_id);
                    ws.clone().send(message);

                    // subscribe to the positions topic to get them back
                    ws.send("positions s".to_string());
                }
                None => info!("No selected route!"),
            };
        })
    };

    // keep processing received messages
    {
        let ws = ws.clone();
        for message in ws.message.iter() {
            info!("received message: {}", message.clone());
        }
    }

    html! {
        <div class="select">
            <select class="select-text" onchange={handle_onchange}>
                <option value="" disabled={true} selected={true}></option>
                <option value="1">{"Route 1"}</option>
                <option value="2">{"Route 2"}</option>
                <option value="3">{"Route 3"}</option>
            </select>

            <span class="select-highlight"></span>
            <span class="select-bar"></span>
            <label class="select-label">{"Select route"}</label>

            <button onclick={request_route}>{"Send route through websocket"}</button>
        </div>
    }
}
