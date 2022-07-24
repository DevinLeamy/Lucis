use crate::common::Color;
use crate::hittable::*;
use crate::ray::Ray;

// CLEAN: ideally, we want all materials in ray_tracer::material::...
pub use crate::ray_tracer::lambertian::*;

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        bounced_ray: &mut Ray,
    ) -> bool;
}
