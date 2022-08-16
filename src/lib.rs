use components::RayTracerDisplay;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

pub mod components;
pub mod utils;

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
