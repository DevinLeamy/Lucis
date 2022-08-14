use components::RayTracerDisplay;
use wasm_bindgen::{prelude::wasm_bindgen};
use web_sys::console::{self, log_1, log_0};
use yew::prelude::*;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Ray Tracer"}</h1>
            <RayTracerDisplay /> 
        </div>
    }
}

#[wasm_bindgen]
pub fn launch_yew() {
    log_1(&"Hello".into());
    yew::Renderer::<App>::new().render();
}
