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
    lens_radius: f64,
    target: Vec3,
    u: Vec3,
    v: Vec3,
}

impl Camera {
    pub fn new(
        origin: Point,
        look_at: Point,
        world_up: Vec3,
        vertical_fov_in_degrees: f64,
        aspect_ratio: f64,
        aperture_diameter: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vertical_fov_in_degrees.to_radians();
        let height = (theta / 2.0).tan();

        let focal_length = 1.0;
        let viewport_height = 2.0 * height;
        let viewport_width: f64 = viewport_height * aspect_ratio;

        let target = Vec3::normalized(origin - look_at); // points in the +z
        let u = Vec3::normalized(Vec3::cross(&world_up, &target)); // horizontal unit vector
        let v = Vec3::normalized(Vec3::cross(&target, &u));

        let horizontal = u * (viewport_width as f64) * focus_distance;
        let vertical = v * (viewport_height as f64) * focus_distance;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - target * focus_distance;

        let lens_radius = aperture_diameter / 2.0;

        Self {
            aspect_ratio,
            focal_length,
            origin,
            viewport_height,
            viewport_width,
            horizontal,
            vertical,
            lower_left,
            lens_radius,

            target,
            u,
            v,
        }
    }

    pub fn aspect_ratio(&self) -> f64 {
        self.aspect_ratio
    }

    pub fn create_ray(&self, h_offset: f64, v_offset: f64) -> Ray {
        let lens_sample = sample_unit_disk() * self.lens_radius;
        let offset = self.u * lens_sample.x() + self.v * lens_sample.y();

        Ray::new(
            self.origin + offset,
            self.lower_left + self.horizontal * h_offset + self.vertical * v_offset
                - self.origin
                - offset,
        )
    }
}
