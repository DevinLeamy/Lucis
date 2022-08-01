use yew::prelude::*;

use common::*;
use components::RayTracerDisplay;
use hittable_list::HittableList;
use ray_tracer::*;
use scenes::*;
use sphere::*;

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
