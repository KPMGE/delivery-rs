use yew::{function_component, Html, html};

#[function_component]
pub fn Select() -> Html {
    html! {
        <div class="select">
            <select class="select-text">
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
