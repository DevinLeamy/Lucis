use std::cell::RefCell;
use std::rc::Rc;

use crate::common::Color;
use crate::hittable::*;
use crate::ray::Ray;

// CLEAN: ideally, we want all materials in ray_tracer::material::...
pub use crate::ray_tracer::lambertian::*;

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<(Color, Ray)>;
}

pub fn make_shared_material<T>(material: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(material))
}
