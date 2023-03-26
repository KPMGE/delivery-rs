use gloo_net::http::Request;
use log::info;
use web_sys::{Event, HtmlSelectElement};
use yew::prelude::*;
use yew_hooks::prelude::*;

use crate::models::Route;

#[function_component]
pub fn Select() -> Html {
    let ws = use_websocket("ws://localhost:3012".to_string());
    let selected_value = use_state::<Option<i32>, _>(|| None);
    let routes = use_state::<Vec<Route>, _>(|| vec![]);

    {
        let routes = routes.clone();
        use_effect_with_deps(move |_| {
            let routes = routes.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_routes: Vec<Route> = Request::get("http://localhost:8000/routes")
                    .send()
                    .await    
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                info!("{:?}", fetched_routes);
                routes.set(fetched_routes);
            });
        }, ());
    }

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

    let route_options = routes
        .iter()
        .enumerate()
        .map(|(idx, route)| html! {
            <option key={idx} value={format!("{}", idx + 1)}>{route.title.clone()}</option>
        })
        .collect::<Html>();

    html! {
        <div class="select">
            <select class="select-text" onchange={handle_onchange}>
                <option value="" disabled={true} selected={true}></option>
                { route_options }
            </select>

            <span class="select-highlight"></span>
            <span class="select-bar"></span>
            <label class="select-label">{"Select route"}</label>

            <button onclick={request_route}>{"Send route through websocket"}</button>
        </div>
    }
}
