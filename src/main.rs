pub use wasm_bindgen_rayon::init_thread_pool;

use components::RayTracerDisplay;
use ray_tracer::*;
use scenes::*;
use yew::prelude::*;

use std::thread;
pub use wasm_bindgen_rayon::init_thread_pool;

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

    log::info!("HERE");
    log::info!("Sum ({})", sum(&[5, 6, 7]));

    // let mut threads = vec![];

    // for i in 0..5 {
    //     threads.push(thread::spawn(move || log::info!("Thread: {}", i)));
    // }

    // for thread in threads {
    //     let _res = thread.join();
    // }
    // yew::start_app::<App>();
}
#[wasm_bindgen]
pub fn sum(numbers: &[i32]) -> i32 {
    numbers.par_iter().sum()
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
