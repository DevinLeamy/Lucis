use super::material::*;
use crate::common::*;
use crate::hittable::HitRecord;
use crate::ray::Ray;

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
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
        *bounced_ray = Ray::new(hit_record.point, reflected_ray);
        *attenuation = self.albedo;

        Vec3::dot(&bounced_ray.direction(), &hit_record.normal) > 0.0
    }
}
