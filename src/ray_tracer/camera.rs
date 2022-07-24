use crate::math::*;
use crate::ray::Ray;
use crate::utils::*;

pub struct Camera {
    aspect_ratio: f64,
    focal_length: f64,
    origin: Point,
    viewport_height: f64,
    viewport_width: f64,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(fov: f64, aspect_ratio: f64) -> Self {
        let theta = fov.to_radians();
        let height = (theta / 2.0).tan();

        let focal_length = 1.0;
        let viewport_height = 2.0 * height;
        let viewport_width: f64 = viewport_height * aspect_ratio;
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

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn create_ray(&self, h_offset: f64, v_offset: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * h_offset + self.vertical * v_offset - self.origin,
        )
    }
}
