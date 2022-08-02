// use crate::common::*;

// pub struct BVHNode {
//     left: Vec<Rc<RefCell<Box<dyn Hittable>>>>,
//     right: Vec<Rc<RefCell<Box<dyn Hittable>>>>,
//     bbox: AABB
// }

// impl BVHNode {
//     pub fn new(objects: HittableList, )
// }

// impl Hittable for BVHNode {
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
//         todo!()
//     }

//     fn bounding_bound(&self, time0: f64, time1: f64) -> Option<AABB> {
//         Some(self.bbox)
//     }
// }
