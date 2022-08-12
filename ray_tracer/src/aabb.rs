use crate::vec3::Vec3;

pub trait Boundable {
    fn bound(&self) -> AABB;
}

pub struct AABB {
    minimum: Vec3,
    maximum: Vec3
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Vec3 { self.minimum }
    pub fn max(&self) -> Vec3 { self.maximum }
}


