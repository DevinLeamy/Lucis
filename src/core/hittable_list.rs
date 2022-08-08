use crate::core::*;

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

    pub fn objects(&self) -> &Vec<Rc<RefCell<Box<dyn Hittable>>>> {
        &self.objects
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut maybe_hit: Option<HitRecord> = None;
        let mut closest_hit = interval.max();

        for hittable in &self.objects {
            /*
            if the hittable object is impacted by a ray sooner
            than any preceding object, the hit is recorded
            and the hit record is updated to hold the new
            closest hit
            */
            if let Some(hit) = hittable.borrow().hit(ray, Interval { min: interval.min(), max: closest_hit }) {
                closest_hit = hit.t();

                maybe_hit = Some(hit);
            }
        }

        maybe_hit
    }

    // fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::common::AABB> {
    //     let mut enclosing_box = None;

    //     for object in self.objects.iter() {
    //         let maybe_aabb = object.as_ref().borrow().bounding_box(time0, time1);

    //         if let Some(bounding_box) = maybe_aabb {
    //             enclosing_box = if enclosing_box.is_none() {
    //                 Some(bounding_box)
    //             } else {
    //                 Some(AABB::bound_aabbs(&enclosing_box.unwrap(), &bounding_box))
    //             }
    //         };
    //     }

    //     enclosing_box
    // }
}

impl Shape for HittableList {
    fn bounding_box(&self) -> Option<AABB> {
        let mut enclosing_box = None;

        for object in self.objects.iter() {
            let maybe_aabb = object.as_ref().borrow().bounding_box();

            if let Some(bounding_box) = maybe_aabb {
                enclosing_box = if enclosing_box.is_none() {
                    Some(bounding_box)
                } else {
                    Some(AABB::bound_aabbs(&enclosing_box.unwrap(), &bounding_box))
                }
            };
        }

        enclosing_box
    }
}

