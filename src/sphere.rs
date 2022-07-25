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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
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
            None
        } else {
            let mut root = (-half_b - discriminant.sqrt()) / a;

            if root < t_min || root > t_max {
                root = (-half_b + discriminant.sqrt()) / a;
                if root < t_min || root > t_max {
                    return None;
                }
            }

            let outer_normal = (ray.position_at(root) - self.center) / self.radius;

            let mut hit_record = HitRecord {
                point: ray.position_at(root),
                normal: outer_normal,
                t: root,
                material: Some(self.material.clone()),
                hit_front_face: false,
            };

            hit_record.set_face_normal(ray, &outer_normal);

            Some(hit_record)
        }
    }
}

pub fn make_shared_hittable<T>(hittable: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(hittable))
}
