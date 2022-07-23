use crate::math::*;
use crate::ray::Ray;
use crate::utils::*;

pub struct Camera {
    aspect_ratio: f32,
    focal_length: f32,
    origin: Point,
    viewport_height: f32,
    viewport_width: f32,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

impl Default for Camera {
    fn default() -> Self {
        let aspect_ratio: f32 = 16.0 / 9.0;
        let focal_length = 1.0;
        let viewport_height: f32 = 2.0;
        let viewport_width: f32 = viewport_height * aspect_ratio;
        let origin = Point::ZEROS();

        let horizontal = Vec3::new(viewport_width as f64, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height as f64, 0.0);
        let lower_left =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length as f64);

        Self {
            aspect_ratio,
            focal_length,
            origin,
            viewport_height,
            viewport_width,
            horizontal,
            vertical,
            lower_left,
        }
    }
}

impl Camera {
    pub fn aspect_ratio(&self) -> f32 {
        self.aspect_ratio
    }

    pub fn create_ray(&self, h_offset: f64, v_offset: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * h_offset + self.vertical * v_offset - self.origin,
        )
    }
}
