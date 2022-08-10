use components::RayTracerDisplay;
use js_sys::Promise;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};
use web_sys::console::log_1;
use yew::prelude::*;

use crate::pool::WorkerPool;

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
            <RayTracerDisplay /> // ..props.clone() />
        </div>
    }
}

#[wasm_bindgen]
pub fn launch_yew() {
    yew::start_app::<App>();
}

#[wasm_bindgen]
pub fn test_on_click() {
    // log_1(&"(Rust) Registered click".into());
}

// #[wasm_bindgen]
// pub fn test_pass_workers(num: u32) {
//     // log_1(&pool.size().into());
//     log_1(&"(Rust)".into());
//     log_1(&num.into());
// }


#[wasm_bindgen]
pub struct Renderer {
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Renderer, JsValue> {
        Ok(Renderer {})
    }

    pub fn test_pass_workers(self, pool: &WorkerPool) { 
        log_1(&"(Rust)".into());
        log_1(&pool.size().into());
    }
}
