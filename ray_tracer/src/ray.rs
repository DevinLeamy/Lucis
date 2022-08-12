use crate::vec3::Vec3;

#[derive(Default, Copy, Clone, PartialEq)]
#[readonly::make]
pub struct Ray {
    /// origin of the ray
    origin: Vec3,
    /// unit vector for the direction of the ray
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Ray { origin, direction }
    }

    pub fn origin(&self) -> Vec3 { self.origin }
    pub fn direction(&self) -> Vec3 { self.direction }

    pub fn position_at(&self, time: f64) -> Vec3 {
        self.origin + self.direction * time
    }
}
