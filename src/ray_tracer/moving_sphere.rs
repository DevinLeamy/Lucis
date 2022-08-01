use crate::common::*;

pub struct MovingSphere {
    time0: f64,
    time1: f64,
    /// initial position
    center0: Point,
    /// final position
    center1: Point,
    radius: f64,
    material: Rc<RefCell<Box<dyn Material>>>,
}

impl MovingSphere {
    pub fn new(
        center0: Point,
        center1: Point,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Rc<RefCell<Box<dyn Material>>>,
    ) -> Self {
        MovingSphere {
            time0,
            time1,
            center0,
            center1,
            radius,
            material,
        }
    }

    /// compute the center of the sphere at _time_
    pub fn center(&self, time: f64) -> Point {
        // linear interpolation
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // center of the sphere at the ray's instant
        let center = self.center(ray.time());
        let center_to_origin = ray.origin() - center;

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

            let outer_normal = (ray.position_at(root) - center) / self.radius;

            let hit_record = HitRecord::new(ray, outer_normal, root, Some(self.material.clone()));
            Some(hit_record)
        }
    }
}
