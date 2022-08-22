use crate::{vec3::Vec3, ray::Ray};

pub trait Boundable {
    fn bound(&self) -> AABB;
}

pub struct AABB {
    minimum: Vec3,
    maximum: Vec3
}

impl AABB {
    pub fn new(minimum: Vec3, maximum: Vec3) -> AABB {
        AABB { minimum, maximum }
    }

    pub fn min(&self) -> Vec3 { self.minimum }
    pub fn max(&self) -> Vec3 { self.maximum }

    // doesn't implemenent Collidable because this collision function
    // ONLY checks if a collision occurs, and doesn't produce a collision
    // record

    /// check for collisions, using the slab method
    pub fn collide(&self, ray: Ray) -> bool {
        let mut t_min = 0.0; 
        let mut t_max = f64::MAX; 

        for i in 0..3 as usize {
            /*
            The ray does not change its position along the i axis. Therefore,
            the origin of the ray determines whether the ray is going to be.
            If the ray is inside the interval, then it could intersect on
            a different axis.
            */
            if ray.direction[i] == 0f64 {
                if ray.origin[i] < self.minimum[i] || self.maximum[i] <= ray.origin[i] {
                    return false;
                }
                continue
            } 

            // compute intersection times
            let mut t0 = (self.minimum[i] - ray.origin[i]) / ray.direction[i];
            let mut t1 = (self.maximum[i] - ray.origin[i]) / ray.direction[i];
            
            (t0, t1) = (f64::min(t0, t1), f64::max(t0, t1));

            t_min = f64::max(t_min, t0);
            t_max = f64::min(t_max, t1);

            if t_min > t_max {
                return false;
            }
        }


        // intersection times all overlap 
        true
    }

    /*
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
    
    */
}

