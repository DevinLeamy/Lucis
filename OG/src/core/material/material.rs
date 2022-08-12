use crate::core::*;

pub struct Scatter {
    pub ray: Ray,
    pub color: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter>;
}
