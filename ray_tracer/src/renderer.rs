use crate::camera::Camera;
use crate::image::{Color, Image};
use crate::ray::Ray;

trait Render {
    fn render_scene(scene: &Scene, perspective: Camera, width: u32, height: u32) -> Image; 
}

struct RayTracer {
}

impl RayTracer {
    fn compute_ray_color(&self, scene: &Scene, ray: Ray) -> Color {
        todo!();
    }
}

impl Render for RayTracer {
    fn render_scene(scene: &Scene, perspective: Camera, width: u32, height: u32) -> Image {
        todo!()
    }
}

