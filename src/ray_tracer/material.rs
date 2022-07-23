use crate::common::Color;
use crate::hittable::*;
use crate::ray::Ray;

pub trait Material {
    fn scatter<'a>(
        &self,
        ray: &'a Ray,
        hit_record: &HitRecord,
        attenuation: Color,
        bounced_ray: &'a mut Ray,
    ) -> bool;
}
