use components::RayTracerDisplay;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1;
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
    yew::Renderer::<App>::new().render();
}
