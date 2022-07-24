use std::cell::RefCell;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::math::Vec3;
use crate::ray::Ray;
use crate::ray_tracer::Material;
use crate::utils::Point;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<RefCell<Box<dyn Material>>>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<RefCell<Box<dyn Material>>>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let center_to_origin = ray.origin() - self.center;

        // application of the quadratic formula
        let a = ray.direction().length_squared();
        let half_b = Vec3::dot(&center_to_origin, &ray.direction());
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if a == 0.0 {
            panic!("Error: division by zero");
        }

        // CLEAN: make this more self-explanatory
        if discriminant < 0.0 {
            false
        } else {
            let mut root = (-half_b - discriminant.sqrt()) / a;

            if root < t_min || root > t_max {
                root = (-half_b + discriminant.sqrt()) / a;
                if root < t_min || root > t_max {
                    return false;
                }
            }
            hit_record.t = root;
            hit_record.point = ray.position_at(hit_record.t);
            let outward_normal = (hit_record.point - self.center) / self.radius;
            hit_record.set_face_normal(ray, &outward_normal);
            hit_record.material = Some(self.material.clone());

            true
        }
    }
}
