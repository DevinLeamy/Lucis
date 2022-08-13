use scene::Scene;
use renderer::{Render, RayTracer};
use camera::Camera;
use image::{WritePPM};

pub mod vec3;
pub mod ray;
pub mod utils;
pub mod image;
pub mod renderer;
pub mod scene;
pub mod camera;
pub mod shape;
pub mod aabb;
pub mod collisions;
pub mod material;
pub mod texture;

fn main() {
    let scene = Scene::materials();   
    let image_width = 600;
    
    let image = RayTracer::render_scene(&scene, Camera::default(), image_width, (image_width as f64 * 9.0 / 16.0) as u32); 

    image.write_as_ppm();
}
