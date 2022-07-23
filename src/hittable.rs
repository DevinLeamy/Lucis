use crate::math::Vec3;
use crate::ray::Ray;
use crate::utils::Point;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub point: Point,
    pub normal: Vec3,
    pub t: f64,
    pub hit_front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        /*
        if the vectors point in opposite directions (dot < 0), the ray must be
        coming from outside of the object.
        */
        self.hit_front_face = Vec3::dot(&ray.direction(), outward_normal) < 0.0;
        self.normal = if self.hit_front_face {
            outward_normal.clone()
        } else {
            -outward_normal.clone()
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}
