use super::material::*;
use crate::common::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        hit_record: &HitRecord,
        attenuation: &mut Color,
        bounced_ray: &mut Ray,
    ) -> bool {
        let reflected_ray = reflect(&ray.direction(), &hit_record.normal);
        *bounced_ray = Ray::new(
            hit_record.point,
            /*
            add fuzziness to reflection my perturbing the reflected
            ray by selecting a target point inside a sphere of radius (fuzz)
            centered at the non-perturbed rays target
            */
            reflected_ray + sample_unit_sphere() * self.fuzz,
        );
        *attenuation = self.albedo;

        Vec3::dot(&bounced_ray.direction(), &hit_record.normal) > 0.0
    }
}
