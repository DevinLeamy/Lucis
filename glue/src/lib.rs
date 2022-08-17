use js_sys::Promise;
use wasm_bindgen::prelude::*;
use ray_tracer::{Camera, WorkerPool, Scene, RayTracer};
use web_sys::console::log_1;

const ASPECT: f64 = 1.0;
const CANVAS_WIDTH: u32 = 600;
const CANVAS_HEIGHT: u32 = (CANVAS_WIDTH as f64 / ASPECT) as u32;

pub fn log(s: String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    alert("Big computation in Rust");
}

#[wasm_bindgen]
pub struct RequestEmitter { }

#[allow(dead_code)]
#[allow(unused_variables)]
#[wasm_bindgen]
impl RequestEmitter {
    #[wasm_bindgen(constructor)] 
    pub fn new() -> Result<RequestEmitter, JsValue> {
        Ok(RequestEmitter {})
    }

    /// request an image to the rendered
    /// returns a callback to the resulting, serialized, image
    pub fn send_request(&self, pool: &WorkerPool) -> Result<Promise, JsValue> {
        RayTracer::render_scene_wasm(Scene::default(), Camera::default(), CANVAS_WIDTH, CANVAS_HEIGHT, pool)
    }

    /// display a serialized image 
    pub fn display_image(&self, image: &JsValue) {
        log("Displaying image".to_string());
    }
}
