use yew::prelude::*;

use components::RayTracerDisplay;
pub use worlds::*;
// use scenes::*;

mod components;
mod cuboid;
mod math;
mod utils;
mod core;
// mod scenes;
mod worlds;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1>{"Ray Tracer"}</h1>
            <RayTracerDisplay />
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
/*
-- Simple Scene (200px width, Aspect [16/9], 50 SAMPLES, 50 MAX_BOUNCE_DEPTH)
Threads: Time (s)
1:       [10.43, 10.52]
5:       [11.12]
8:       [11.72, 11.81]
9:       [13.35]
10:      [14.21]
11:      [15.14]
20:      [26.96] (and it slows down the computer)

-- Complex Scene (200px width, Aspect [16/9], 50 SAMPLES, 50 MAX_BOUNCE_DEPTH)
Threads: Time (s)
1:       [213.11]
8:       [237.16]
*/
