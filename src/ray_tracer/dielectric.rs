use crate::{common::*, hittable::HitRecord, ray::Ray};

use super::Material;

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Schlick's approximation to reflectance
        let x = (1.0 - refractive_index) / (1.0 + refractive_index);

        x * x + (1.0 - x * x) * f64::powf(1.0 - cosine, 5.0)
    }
}

/*
Dielectric material that refracts all incoming light
*/
impl Material for Dielectric {
    fn scatter(&self, ray: &crate::ray::Ray, hit_record: &HitRecord) -> Option<(Color, Ray)> {
        let attenuation = Color::ONES();

        let refractive_ratio = if hit_record.hit_front_face() {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = Vec3::normalized(ray.direction());

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &hit_record.normal()), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refractive_ratio * sin_theta > 1.0;

        let out_direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refractive_ratio) > random_float()
        {
            reflect(&unit_direction, &hit_record.normal())
        } else {
            refract(&unit_direction, &hit_record.normal(), refractive_ratio)
        };

        let bounced_ray = Ray::new(hit_record.point(), out_direction);

        Some((attenuation, bounced_ray))
    }
}
