use crate::collisions::{CollisionRecord};
use crate::image::Color;
use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct CollisionResult {
    pub reflected_ray: Ray,
    pub color: Color
}

pub trait Material {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult;
    fn as_mat(self) -> Box<dyn Material>;
}

#[derive(Copy, Clone)]
pub enum MaterialType {
    Lambertian(Lambertian), 
    Dielectric(Dielectric)
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    color: Color
}

impl Lambertian {
    pub fn new(color: Color) -> Lambertian {
        Lambertian { color }
    }
}

impl Material for Lambertian {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        let mut bounce_dir = collision.normal + Vec3::random().normalize();

        if bounce_dir.near_zero() {
            bounce_dir = collision.normal;
        }

        CollisionResult {
            reflected_ray: Ray::new(collision.point, bounce_dir),
            color: self.color
        }
    }

    fn as_mat(self) -> Box<dyn Material> {
        Box::new(self) 
    }
}

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Dielectric {
        Dielectric { ref_index }
    }

    fn reflectance(cos: f64, ref_ratio: f64) -> f64 {
        // Schlick's approximation to reflectance
        let x = (1.0 - ref_ratio) / (1.0 + ref_ratio);
        x * x + (1.0 - x * x) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        let ref_ratio = 1.0 / self.ref_index;

        // let refractive_ratio = if hit_record.hit_front_face() {
        //     1.0 / self.refractive_index
        // } else {
        //     self.refractive_index
        // };

        let cos = f64::min(Vec3::dot(-ray.direction, collision.normal), 1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let cannot_refract = ref_ratio * sin > 1.0;
        let veto_refract = Dielectric::reflectance(cos, ref_ratio) > crate::utils::random_float(); 

        let reflect = cannot_refract || veto_refract;

        CollisionResult {
            reflected_ray: match reflect { 
                true  => ray.reflect(collision.normal, collision.point),
                false => ray.refract(collision.normal, collision.point, ref_ratio)
            },
            color: Color::new(1.0, 1.0, 1.0),
        }
    }

    fn as_mat(self) -> Box<dyn Material> {
        Box::new(self)
    }
}

