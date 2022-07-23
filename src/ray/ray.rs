use crate::math::Vec3;
use crate::utils::point::*;

#[derive(Default)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Ray { origin, direction }
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
