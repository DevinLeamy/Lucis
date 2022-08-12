use crate::core::*;

#[derive(Default)]
pub struct Ray {
    /// origin of the ray
    origin: Point,
    /// unit vector for the direction of the ray
    direction: Vec3,
    /// instant when the ray exists
    time: f64,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray::new_instant(origin, direction, 0f64)
    }

    pub fn new_instant(origin: Point, direction: Vec3, time: f64) -> Self {
        Ray {
            origin,
            direction: Vec3::normalized(direction),
            time,
        }
    }

    pub fn set_time(&mut self, time: f64) {
        self.time = time;
    }

    pub fn time(&self) -> f64 {
        self.time
    }

    pub fn origin(&self) -> Point {
        self.origin
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn position_at(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }
}
