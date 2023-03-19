use log::info;
use web_sys::{Event, HtmlSelectElement};
use yew::prelude::*;

#[function_component]
pub fn Select() -> Html {
    let selected_value = use_state::<Option<i32>, _>(|| None);

    let handle_onchange = {
        move |e: Event| {
            if let Some(target) = e.target_dyn_into::<HtmlSelectElement>() {
                let value = target.value().parse::<i32>().unwrap();
                selected_value.set(Some(value));
                info!("selected value: {}", value);
            }
        }
    };

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
        </div>
    }
}
