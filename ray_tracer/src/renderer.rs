use crate::{Element, ElementId};
use crate::camera::Camera;
use crate::collisions::{CollisionRecord, Collidable};
use crate::material::{MaterialType, Material};
use crate::image::{Color, Image, ColorU8};
use crate::ray::Ray;
use crate::scene::{Scene};
use crate::shape::{ShapeType};
use crate::utils::random_float;
use crate::pool::WorkerPool;

use js_sys::Promise;
use rayon::prelude::*;
use futures::channel::oneshot;
use wasm_bindgen::JsValue;

const MAX_BOUNCE_DEPTH: u32 = 50;
const SAMPLES_PER_PIXEL: u32 = 20;
const MIN_INTERSECTION_T: f64 = 0.001;

pub trait Render {
    fn render_scene(scene: &Scene, camera: Camera, width: u32, height: u32) -> Image; 
}

pub struct RayTracer {}

impl RayTracer {
    fn compute_ray_color(scene: &Scene, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == MAX_BOUNCE_DEPTH {
            return Color::black();
        }

        if let Some((element, record)) = RayTracer::compute_collision(scene, ray) {
            let result = element.material.resolve(ray, record);

            result.color * RayTracer::compute_ray_color(scene, result.reflected_ray, bounce_depth + 1)
        } else {
            return Color::white();
        }
    }
}

impl Render for RayTracer {
    fn render_scene(scene: &Scene, camera: Camera, width: u32, height: u32) -> Image {
        let mut image = Image::new(height, width); 

        let pixels = width * height;

        let indices = (0..pixels).collect::<Vec<u32>>();

        let mut colors = vec![];

        indices.par_iter().map(|i| {
            let row = i / width;
            let col = i % width;

            let mut acc_color = Color::black();

            for _ in 0..SAMPLES_PER_PIXEL {
                let row_s = row as f64 + random_float();
                let col_s = col as f64 + random_float();

                // convert pixel coordinate to world coordinates
                let world_x = col_s / (width - 1) as f64;
                let world_y = row_s / (height - 1) as f64; 

                let ray = camera.create_ray(world_x, world_y);

                let color = RayTracer::compute_ray_color(scene, ray, 0);

                acc_color += color; 
            }

            let normalized = Color::new(
                acc_color.red / SAMPLES_PER_PIXEL as f64,
                acc_color.green / SAMPLES_PER_PIXEL as f64,
                acc_color.blue / SAMPLES_PER_PIXEL as f64,
            ).gamma_corrected(); 

            normalized
        }).collect_into_vec(&mut colors);

        indices.iter().for_each(|i| {
            let row = i / width;
            let col = i % width;
            
            image.set_color(row, col, ColorU8::from(colors[*i as usize]))
        });

        image
    }
}
impl RayTracer {
    pub fn render_scene_wasm(scene: Scene, camera: Camera, width: u32, height: u32, pool: &WorkerPool) -> Result<Promise, JsValue> {
        let pixels = width * height;
        let indices = (0..pixels).collect::<Vec<u32>>();
        let mut colors = vec![Color::black(); pixels as usize];

        let thread_pool = rayon::ThreadPoolBuilder::new()
            .num_threads(pool.size())
            .spawn_handler(|thread| Ok(pool.run(|| thread.run()).unwrap()))
            .build()
            .unwrap();
        
        let (sender, receiver) = oneshot::channel();

        pool.run(move || {
            thread_pool.install(|| {
                indices.par_iter().map(|i| {
                    let row = i / width;
                    let col = i % width;
        
                    let mut acc_color = Color::black();
        
                    for _ in 0..SAMPLES_PER_PIXEL {
                        let row_s = row as f64 + random_float();
                        let col_s = col as f64 + random_float();
        
                        // convert pixel coordinate to world coordinates
                        let world_x = col_s / (width - 1) as f64;
                        let world_y = row_s / (height - 1) as f64; 
        
                        let ray = camera.create_ray(world_x, world_y);
        
                        let color = RayTracer::compute_ray_color(&scene, ray, 0);
        
                        acc_color += color; 
                    }
        
                    let normalized = Color::new(
                        acc_color.red / SAMPLES_PER_PIXEL as f64,
                        acc_color.green / SAMPLES_PER_PIXEL as f64,
                        acc_color.blue / SAMPLES_PER_PIXEL as f64,
                    ).gamma_corrected(); 
        
                    normalized
                }).collect_into_vec(&mut colors);
            });

            drop(sender.send(colors));
        })?;

        let render_complete = async move {
            match receiver.await {
                Ok(colors) => { 
                    let mut image = Image::new(height, width); 
                    let indices = (0..pixels).collect::<Vec<u32>>();
                    indices.iter().for_each(|i| {
                        let row = i / width;
                        let col = i % width;
                        
                        image.set_color(row, col, ColorU8::from(colors[*i as usize]))
                    });
                    Ok(JsValue::from_serde(&image).unwrap())
                },
                Err(_) => Err(JsValue::undefined())
            }
        };

        Ok(wasm_bindgen_futures::future_to_promise(render_complete))
    }
}

impl RayTracer {
    fn compute_collision(scene: &Scene, ray: Ray) -> Option<(Element, CollisionRecord)> {
        let mut c_record: Option<CollisionRecord> = None;
        let mut c_t = f64::MAX;
        let mut c_element: Option<Element> = None;

        scene.objects.iter().for_each(|element| {
            if let Some(record) = element.collide(ray) {
                // update the collision record if 
                // the ray collides earlier
                if MIN_INTERSECTION_T < record.t && record.t < c_t { 
                    c_t = record.t;
                    c_record = Some(record);
                    c_element= Some(element.clone());
                } 
            };
        });

        if c_element.is_none() {
            return None
        }

        Some((c_element.unwrap(), c_record.unwrap()))
    } 

    pub fn compute_collision_element(scene: &Scene, ray: Ray) -> Option<Element> {
        match RayTracer::compute_collision(scene, ray) {
            Some((element, _)) => Some(element),
            None               => None
        }
    }
}

