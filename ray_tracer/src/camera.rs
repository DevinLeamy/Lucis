use crate::vec3::Vec3;
use crate::ray::Ray;

pub struct CameraConfig {
    /// aspect ratio (horizontal / vertical)
    pub aspect: f64,
    /// origin in world coordinates
    pub origin: Vec3,
    /// look at vector
    pub look_at: Vec3,
    /// unit vector for world's vertical axis
    pub world_up: Vec3,
    /// distance to "focus"
    pub focus_dist: f64,
    /// aperture diameter
    pub aperture: f64,
    /// vertical field of view in degrees
    pub vertical_fov_degrees: f64,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            aspect: 16.0 / 9.0,
            origin: Vec3::new(13.0, 2.0, 3.0),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            focus_dist: 10.0,
            aperture: 0.1,
            vertical_fov_degrees: 20.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    aspect: f64,
    focal_length: f64,
    origin: Vec3,
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
    pub fn new(cfg: CameraConfig) -> Self {
        let theta = cfg.vertical_fov_degrees.to_radians();
        let height = (theta / 2.0).tan();

        let focal_length = 1.0;
        let viewport_height = 2.0 * height;
        let viewport_width: f64 = viewport_height * cfg.aspect;

        let target = (cfg.origin - cfg.look_at).normalize(); // points in the +z
        let u = (Vec3::cross(cfg.world_up, target)).normalize(); // horizontal unit vector
        let v = (Vec3::cross(target, u)).normalize();

        let horizontal = u * (viewport_width as f64) * cfg.focus_dist;
        let vertical = v * (viewport_height as f64) * cfg.focus_dist;
        let lower_left = cfg.origin - horizontal / 2.0 - vertical / 2.0 - target * cfg.focus_dist;

        let lens_radius = cfg.aperture / 2.0;

        Self {
            aspect: cfg.aspect,
            focal_length,
            origin: cfg.origin,
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

    pub fn aspect(&self) -> f64 { self.aspect }

    pub fn create_ray(&self, h_offset: f64, v_offset: f64) -> Ray {
        // note sure what this is supposed to do
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
