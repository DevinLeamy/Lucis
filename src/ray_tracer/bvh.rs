use crate::common::*;

/// Bounding Volume Hierarchy
pub struct BVHNode {
    left: Rc<RefCell<Box<dyn Hittable>>>,
    right: Rc<RefCell<Box<dyn Hittable>>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(objects: HittableList, time0: f64, time1: f64) -> BVHNode {
        BVHNode::from_list(objects.objects(), time0, time1)
    }

    pub fn from_list(
        objects: &Vec<Rc<RefCell<Box<dyn Hittable>>>>,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let axis = random_natural(0, 2);
        if objects.len() == 1 {
            return BVHNode {
                left: Rc::clone(&objects[0]),
                right: Rc::clone(&objects[0]),
                bbox: objects[0].borrow().bounding_box(time0, time1).unwrap(),
            };
        }
        let (left_objs, right_objs) = BVHNode::halve_on_axis(axis, objects);

        let left_node = BVHNode::from_list(&left_objs.to_vec(), time0, time1);
        let right_node = BVHNode::from_list(&right_objs.to_vec(), time0, time1);

        let left_box = left_node.bounding_box(time0, time1);
        let right_box = left_node.bounding_box(time0, time1);

        if left_box.is_none() && right_box.is_none() {
            panic!("Error: no bounding boxes to construct BVH")
        }

        let main_box = AABB::bound_aabbs(&left_box.unwrap(), &right_box.unwrap());

        BVHNode {
            left: Rc::new(RefCell::new(Box::new(left_node))),
            right: Rc::new(RefCell::new(Box::new(right_node))),
            bbox: main_box,
        }
    }

    // CLEAN: move partitioning logic somewhere else
    pub fn halve_on_axis(
        axis: u32,
        objects: &Vec<Rc<RefCell<Box<dyn Hittable>>>>,
    ) -> (
        Vec<Rc<RefCell<Box<dyn Hittable>>>>,
        Vec<Rc<RefCell<Box<dyn Hittable>>>>,
    ) {
        let median = (objects.len()) / 2;
        let pivot = BVHNode::quick_select(axis, objects, median);

        BVHNode::partition_on_axis(axis, objects, pivot)
    }

    fn partition_on_axis(
        axis: u32,
        objects: &Vec<Rc<RefCell<Box<dyn Hittable>>>>,
        pivot: usize,
    ) -> (
        Vec<Rc<RefCell<Box<dyn Hittable>>>>,
        Vec<Rc<RefCell<Box<dyn Hittable>>>>,
    ) {
        let mut left = vec![];
        let mut right = vec![];

        // value of the pivot element
        let pivot_obj = &objects[pivot];

        for object in objects.iter() {
            if BVHNode::compare(&pivot_obj, &object, axis) {
                left.push(Rc::clone(object));
            } else {
                right.push(Rc::clone(object));
            }
        }

        (left, right)
    }

    fn quick_select(axis: u32, objects: &Vec<Rc<RefCell<Box<dyn Hittable>>>>, k: usize) -> usize {
        let pivot = random_natural(0, objects.len() as u32) as usize;

        let (left, right) = BVHNode::partition_on_axis(axis, objects, pivot);

        if k == left.len() + 1usize {
            pivot
        } else if k < left.len() + 1usize {
            BVHNode::quick_select(axis, &left, k)
        } else {
            BVHNode::quick_select(axis, &right, k - (left.len() + 1usize))
        }
    }

    fn compare(
        object0: &Rc<RefCell<Box<dyn Hittable>>>,
        object1: &Rc<RefCell<Box<dyn Hittable>>>,
        axis: u32,
    ) -> bool {
        let box0 = object0.as_ref().borrow().bounding_box(0f64, 0f64);
        let box1 = object1.as_ref().borrow().bounding_box(0f64, 0f64);

        if box0.is_none() && box1.is_none() {
            panic!("Error: (compare) Cannot compare objects without bounding boxes.");
        }

        AABB::compare(&box0.unwrap(), &box1.unwrap(), axis)
    }
}

impl Hittable for BVHNode {
    // CLEAN: how to address this compare-values-in-options-if-they-exist pattern?
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(ray, t_min, t_max) {
            return None;
        }

        let left_hit = self.left.as_ref().borrow().hit(ray, t_min, t_max);
        let right_hit = self.right.as_ref().borrow().hit(ray, t_min, t_max);

        if left_hit.is_none() && right_hit.is_none() {
            return None;
        }

        let left_hit_time = if let Some(hit) = &left_hit {
            hit.t()
        } else {
            f64::MAX
        };
        let right_hit_time = if let Some(hit) = &right_hit {
            hit.t()
        } else {
            f64::MAX
        };

        if left_hit_time < right_hit_time {
            left_hit
        } else {
            right_hit
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}
