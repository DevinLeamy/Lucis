use std::cell::RefCell;
use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

/*
aggregates a list of Hittable objects
*/
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut potential_hit: HitRecord = HitRecord::default();
        let mut hit_surface = false;
        let mut closest_hit = t_max;

        for hittable in &self.objects {
            /*
            if the hittable object is impacted by a ray sooner
            than any preceding object, the hit is recorded
            and the hit record is updated to hold the new
            closest hit
            */
            if hittable
                .borrow()
                .hit(ray, t_min, closest_hit, &mut potential_hit)
            {
                hit_surface = true;
                closest_hit = potential_hit.t;

                *hit_record = potential_hit.clone();
            }
        }

        hit_surface
    }
}
