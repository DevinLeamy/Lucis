use crate::common::*;

pub struct Lambertian {
    albedo: Rc<Box<dyn Texture>>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian::from_color(&albedo)
    }

    pub fn from_color(albedo: &Color) -> Lambertian {
        Lambertian::from_texture(Rc::new(Box::new(SolidTexture::new(*albedo))))
    }

    pub fn from_texture(texture: Rc<Box<dyn Texture>>) -> Lambertian {
        Lambertian::from_texture(Rc::clone(&texture))
    }
}

// CLEAN: update scatter to return Option<Ray>, rather than take in a reference and return true
impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = hit_record.normal() + random_unit_vector();

        // guard against potential floating point errors
        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal();
        }

        let bounced_ray = Ray::new_instant(hit_record.point(), scatter_direction, ray.time());
        let attenuation = self.albedo;

        Some(Scatter {
            texture: attenuation,
            ray: bounced_ray,
        })
    }
}
