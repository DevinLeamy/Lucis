use crate::common::*;

pub struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> AABB {
        AABB { minimum, maximum }
    }
}

impl AABB {
    pub fn min(&self) -> Point {
        self.minimum
    }

    pub fn max(&self) -> Point {
        self.maximum
    }

    /// check for collisions, using the slab method
    fn hit(&self, ray: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for i in 0..3 as usize {
            let (t0, t1) = if ray.direction()[i] == 0f64 {
                /*
                The ray does not change its position along the i axis. Therefore,
                the origin of the ray determines whether the ray is going to be.
                If the ray is inside the interval, then it could intersect on
                a different axis.
                */
                if ray.origin()[i] >= self.maximum[i] && ray.origin()[i] < self.minimum[i] {
                    return false;
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
                return false;
            }
        }
        true
    }

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
}
