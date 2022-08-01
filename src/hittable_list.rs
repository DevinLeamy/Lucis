use std::cell::RefCell;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

/*
aggregates a list of Hittable objects
*/
#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Rc<RefCell<Box<dyn Hittable>>>>,
}

impl Default for HittableList {
    fn default() -> Self {
        Self { objects: vec![] }
    }
}

impl HittableList {
    pub fn new(objects: Vec<Rc<RefCell<Box<dyn Hittable>>>>) -> Self {
        HittableList { objects }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Rc<RefCell<Box<dyn Hittable>>>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut maybe_hit: Option<HitRecord> = None;
        let mut closest_hit = t_max;

        for hittable in &self.objects {
            /*
            if the hittable object is impacted by a ray sooner
            than any preceding object, the hit is recorded
            and the hit record is updated to hold the new
            closest hit
            */
            if let Some(hit) = hittable.borrow().hit(ray, t_min, closest_hit) {
                closest_hit = hit.t();
                maybe_hit = Some(hit);
            }
        }

        maybe_hit
    }
}
