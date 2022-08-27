use js_sys::Promise;
use wasm_bindgen::prelude::*;
use ray_tracer::{Camera, Box, WorkerPool, Scene, RayTracer, Element, MaterialType, Metal, Sphere, Vec3, Color, ShapeType, CameraConfig, RayTracerConfig, TextureType, CheckeredTexture, Lambertian};
use web_sys::console::log_1;

use serde::{Deserialize, Serialize};

// const ASPECT: f64 = 1.0;
const CANVAS_WIDTH: u32 = 750; // 600;
const CANVAS_HEIGHT: u32 = 750; // (CANVAS_WIDTH as f64 / ASPECT) as u32;

pub fn log(s: String) {
    log_1(&JsValue::from(s));
}

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
} 

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ClientConfig {
    pub origin: Vec3,
    pub look_at: Vec3,
    pub max_bounce_depth: u32,
    pub samples: u32
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
        RayTracer::new(RayTracerConfig::default()).render_scene_wasm(Scene::default(), Camera::default(), CANVAS_WIDTH, CANVAS_HEIGHT, pool)
    }

    pub fn render_element(&self, element: JsValue, background_mat: JsValue, config: JsValue, pool: &WorkerPool) -> Result<Promise, JsValue> {
        let config = config.into_serde::<ClientConfig>().unwrap();

        let element = element.into_serde().unwrap();

        let background;

        if background_mat == JsValue::UNDEFINED {
            background = MaterialType::Lambertian(Lambertian::new(
                    TextureType::CheckeredTexture(CheckeredTexture::new(
                        Color::white(),
                        Color::new(0.1, 0.1, 0.1),
                    )))
                );
        } else {
            background = background_mat.into_serde().unwrap(); 
        }

        let camera = Camera::new(CameraConfig {
            origin: config.origin,
            look_at: config.look_at,
            aspect: CANVAS_WIDTH as f64 / CANVAS_HEIGHT as f64,
            ..CameraConfig::default()
        });

        let ray_tracer = RayTracer::new(RayTracerConfig {
            max_bounce_depth: config.max_bounce_depth,
            samples: config.samples,
            ..RayTracerConfig::default()
        });

        let scene = Scene::element_with_background(element, background);
        
        ray_tracer.render_scene_wasm(scene, camera, CANVAS_WIDTH, CANVAS_HEIGHT, pool)
    }

    /// TESTING - get serialized element
    pub fn get_element(&self) -> Result<JsValue, JsValue> {
        let element = Element::new( 
            MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.9).into(), 0.2)),
            ShapeType::Sphere(Sphere::new(Vec3::new(-1.0, 0.5, -1.0), 0.5))
        );

        Ok(JsValue::from_serde(&element).unwrap())
    }

    pub fn get_default_box(&self) -> Result<JsValue, JsValue> {
        let new_box = ShapeType::Box(Box::cube(0.8, Vec3::new(0.0, 0.5, 0.0)));
        Ok(JsValue::from_serde(&new_box).unwrap())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn marco() {
    console_error_panic_hook::set_once();
    alert("polo");
}
