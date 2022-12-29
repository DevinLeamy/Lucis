use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct CameraConfig {
    /// aspect ratio (horizontal / vertical)
    pub aspect: f32,
    /// origin in world coordinates
    pub origin: Vec3,
    /// look at vector
    pub look_at: Vec3,
    /// unit vector for world's vertical axis
    pub world_up: Vec3,
    /// distance to "focus"
    pub focus_dist: f32,
    /// vertical field of view in degrees
    pub vertical_fov_degrees: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            aspect: 750.0 / 450.0, // 1.0,// 16.0 / 9.0,
            // origin: Vec3::new(13.0, 3.0, 3.0),
            origin: Vec3::new(0.0, 3.5, 2.5),
            look_at: Vec3::new(0.0, 0.0, 0.0),
            world_up: Vec3::new(0.0, 1.0, 0.0),
            focus_dist: 10.0,
            vertical_fov_degrees: 25.0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Camera {
    aspect: f32,
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left: Vec3,
}

impl Camera {
    pub fn new(cfg: CameraConfig) -> Self {
        let theta = cfg.vertical_fov_degrees.to_radians();
        let height = (theta / 2.0).tan();

        let viewport_height = 2.0 * height;
        let viewport_width: f32 = viewport_height * cfg.aspect;

        let mut target = cfg.origin - cfg.look_at;

        // Don't allow target be a near-zero vector
        if target.length() < 0.001 {
            target += Vec3::new(0.01, 0.0, 0.0);
        }

        let target = target.normalize(); // points in +z
        let u = (Vec3::cross(cfg.world_up, target)).normalize(); // horizontal unit vector
        let v = (Vec3::cross(target, u)).normalize();

        let horizontal = u * (viewport_width as f32) * cfg.focus_dist;
        let vertical = v * (viewport_height as f32) * cfg.focus_dist;
        let lower_left = cfg.origin - horizontal / 2.0 - vertical / 2.0 - target * cfg.focus_dist;

        Self {
            aspect: cfg.aspect,
            origin: cfg.origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn translate(&mut self, translation: Vec3) {
        self.origin += translation;
    }

    pub fn aspect(&self) -> f32 {
        self.aspect
    }
    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn create_ray(&self, h_offset: f32, v_offset: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left + self.horizontal * h_offset + self.vertical * v_offset - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera::new(CameraConfig::default())
    }
}
