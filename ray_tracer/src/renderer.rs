use crate::camera::Camera;
use crate::collisions::{Collidable, CollisionRecord};
use crate::image::{Color, ColorU8, Image};
use crate::material::Material;
use crate::pool::WorkerPool;
use crate::ray::Ray;
use crate::scene::Scene;
use crate::utils::random_float;
use crate::Element;

use futures::channel::oneshot;
use js_sys::Promise;
use rayon::prelude::*;
use wasm_bindgen::JsValue;

const MIN_INTERSECTION_T: f64 = 0.001;

pub trait Render {
    fn render_scene(&self, scene: &Scene, camera: Camera, width: u32, height: u32) -> Image;
}

/// Configuration for the [RayTracer]
pub struct RayTracerConfig {
    /// Maximum number of ray bounces
    pub max_bounce_depth: u32,
    /// Number of samples per pixel
    pub samples: u32,
    /// Background color (the color of a ray when it "hit" nothing)
    pub background_color: Color,
}

impl Default for RayTracerConfig {
    fn default() -> Self {
        Self {
            max_bounce_depth: 3,
            samples: 5,
            background_color: Color::white(),
        }
    }
}

pub struct RayTracer {
    max_bounce_depth: u32,
    samples: u32,
    background_color: Color,
}

impl RayTracer {
    pub fn new(config: RayTracerConfig) -> Self {
        Self {
            max_bounce_depth: config.max_bounce_depth,
            samples: config.samples,
            background_color: config.background_color,
        }
    }

    fn compute_ray_color(&self, scene: &Scene, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == self.max_bounce_depth {
            return Color::black();
        }

        if let Some((element, record)) = self.compute_collision(scene, ray) {
            let result = element.material.resolve(ray, record);

            // this is a hack - see DiffuseLight in material.rs
            result.emitted_light
                + result.color
                    * self.compute_ray_color(scene, result.reflected_ray, bounce_depth + 1)
        } else {
            self.background_color
        }
    }
}

impl Render for RayTracer {
    fn render_scene(&self, scene: &Scene, camera: Camera, width: u32, height: u32) -> Image {
        let mut image = Image::new(height, width);

        let pixels = width * height;

        let indices = (0..pixels).collect::<Vec<u32>>();

        let mut colors = vec![];

        indices
            .par_iter()
            .map(|i| {
                let row = i / width;
                let col = i % width;

                let mut acc_color = Color::black();

                for _ in 0..self.samples {
                    let row_s = row as f64 + random_float();
                    let col_s = col as f64 + random_float();

                    // convert pixel coordinate to world coordinates
                    let world_x = col_s / (width - 1) as f64;
                    let world_y = row_s / (height - 1) as f64;

                    let ray = camera.create_ray(world_x, world_y);

                    let color = self.compute_ray_color(scene, ray, 0);

                    acc_color += color;
                }

                let normalized = Color::new(
                    acc_color.red / self.samples as f64,
                    acc_color.green / self.samples as f64,
                    acc_color.blue / self.samples as f64,
                )
                .gamma_corrected();

                normalized
            })
            .collect_into_vec(&mut colors);

        indices.iter().for_each(|i| {
            let row = i / width;
            let col = i % width;

            image.set_color(row, col, ColorU8::from(colors[*i as usize]))
        });

        image
    }
}
impl RayTracer {
    pub fn render_scene_wasm(
        self,
        scene: Scene,
        camera: Camera,
        width: u32,
        height: u32,
        pool: &WorkerPool,
    ) -> Result<Promise, JsValue> {
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
                indices
                    .par_iter()
                    .map(|i| {
                        let row = i / width;
                        let col = i % width;

                        let mut acc_color = Color::black();

                        for _ in 0..self.samples {
                            let row_s = row as f64 + random_float();
                            let col_s = col as f64 + random_float();

                            // convert pixel coordinate to world coordinates
                            let world_x = col_s / (width - 1) as f64;
                            let world_y = row_s / (height - 1) as f64;

                            let ray = camera.create_ray(world_x, world_y);

                            let color = self.compute_ray_color(&scene, ray, 0);

                            acc_color += color;
                        }

                        let normalized = Color::new(
                            acc_color.red / self.samples as f64,
                            acc_color.green / self.samples as f64,
                            acc_color.blue / self.samples as f64,
                        )
                        .gamma_corrected();

                        normalized
                    })
                    .collect_into_vec(&mut colors);
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
                }
                Err(_) => Err(JsValue::undefined()),
            }
        };

        Ok(wasm_bindgen_futures::future_to_promise(render_complete))
    }
}

impl RayTracer {
    fn compute_collision(&self, scene: &Scene, ray: Ray) -> Option<(Element, CollisionRecord)> {
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
                    c_element = Some(element.clone());
                }
            };
        });

        if c_element.is_none() {
            return None;
        }

        Some((c_element.unwrap(), c_record.unwrap()))
    }

    pub fn compute_collision_element(&self, scene: &Scene, ray: Ray) -> Option<Element> {
        match self.compute_collision(scene, ray) {
            Some((element, _)) => Some(element),
            None => None,
        }
    }
}
