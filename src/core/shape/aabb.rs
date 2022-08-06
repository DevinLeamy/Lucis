use crate::core::*;

#[derive(Clone, Copy)]
pub struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Point { self.minimum }
    pub fn max(&self) -> Point { self.maximum }

    /// construct the bounding box of two bounding boxes
    pub fn bound_aabbs(box0: &AABB, box1: &AABB) -> AABB {
        let mut min = Vec3::ZEROS();
        let mut max = Vec3::ZEROS();

        for i in 0..3 {
            min[i] = f64::min(box0.min()[i], box1.min()[i]);
            max[i] = f64::min(box0.min()[i], box1.min()[i]);
        }

        AABB::new(min, max)
    }

    /// check if box0 compares strickly less than box1 on a given axis based on their minimum value
    pub fn compare(box0: &AABB, box1: &AABB, axis: u32) -> bool {
        box0.min()[axis as usize] < box1.min()[axis as usize]
    }
}

impl Shape for AABB {
    fn bounding_box(&self) -> Option<AABB> {
        Some(*self)
    }
}

impl Hittable for AABB {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let mut t_min = interval.min();
        let mut t_max = interval.max();
        for i in 0..3 as usize {
            let (t0, t1) = if ray.direction()[i] == 0f64 {
                /*
                The ray does not change its position along the i axis. Therefore,
                the origin of the ray determines whether the ray is going to be.
                If the ray is inside the interval, then it could intersect on
                a different axis.
                */
                if ray.origin()[i] >= self.maximum[i] && ray.origin()[i] < self.minimum[i] {
                    return None;
                }

                (ray.time(), ray.time())
            } else {
                let t0 = (self.minimum[i] - ray.origin()[i]) / ray.direction()[i];
                let t1 = (self.maximum[i] - ray.origin()[i]) / ray.direction()[i];

                (f64::min(t0, t1), f64::max(t0, t1))
            };

            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);

            if t_min >= t_max {
                return None;
            }
        }
        // should be some true/false 
        Some(HitRecord::default())
    }
}
