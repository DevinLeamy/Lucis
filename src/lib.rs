use components::RayTracerDisplay;
use ray_tracer::*;
use scenes::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

mod common;
mod components;
mod cuboid;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod ray_tracer;
mod scenes;
mod sphere;
mod utils;

mod pool;

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
    yew::start_app::<App>();
}
