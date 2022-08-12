use components::RayTracerDisplay;
use wasm_bindgen::{prelude::wasm_bindgen};
use yew::prelude::*;

mod perlin;
mod core;
mod components;
mod cuboid;
mod math;
mod utils;
mod worlds;

pub mod pool;

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
