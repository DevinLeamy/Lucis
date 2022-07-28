use super::material::*;
use crate::common::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

// CLEAN: update scatter to return Option<Ray>, rather than take in a reference and return true
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hit_record.normal + random_unit_vector();

        // guard against potential floating point errors
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        let bounced_ray = Ray::new(hit_record.point, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, bounced_ray))
    }
}