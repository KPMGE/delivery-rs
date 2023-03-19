use log::info;
use yew::{function_component, Html, html};

#[function_component]
pub fn Select() -> Html {
    wasm_logger::init(wasm_logger::Config::default());

    html! {
        <div class="select">
            <select class="select-text" onchange={|_| info!("the value has changed!")}>
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
