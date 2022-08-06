use crate::common::*;

pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<RefCell<Box<dyn Material>>>,
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<RefCell<Box<dyn Material>>>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let center_to_origin = ray.origin() - self.center;

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

            let intersection_point = ray.position_at(root);

            let outer_normal = (intersection_point - self.center) / self.radius;
            let t_coord = self.map(&outer_normal);

            let hit_record = HitRecord::new(
                ray,
                outer_normal,
                root,
                Some(self.material.clone()),
                Some(t_coord),
            );
            Some(hit_record)
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<crate::common::AABB> {
        let min = self.center - Vec3::ONES() * self.radius;
        let max = self.center + Vec3::ONES() * self.radius;
        Some(AABB::new(min, max))
    }
}

impl TextureMap for Sphere {
    fn map(&self, p: &Point) -> TextureCoord {
        let theta = -p.y().acos();
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        TextureCoord {
            u: phi / (2f64 * PI),
            v: theta / PI,
        }
    }
}

pub fn make_shared_hittable<T>(hittable: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(hittable))
}
