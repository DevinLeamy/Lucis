use std::borrow::Borrow;

use crate::core::*;

pub trait Shape {
    fn bounding_box(&self) -> Option<AABB>;
}

pub trait Hittable: Shape {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord>;
}


impl Hittable for RSphere {
    fn hit(&self, ray: &Ray, interval: Interval) -> Option<HitRecord> {
        let center = self.sphere.center();
        let intersection_time = match intersection(center, self.sphere.radius(), ray) {
            Some((r1, r2)) => {
                if interval.contains(r1) { r1 } 
                else if interval.contains(r2) { r2 } 
                else { return None; }
            }
            _ => return None,
        };

        let intersection_point = ray.position_at(intersection_time);
        let normal = self.sphere.surface_normal(intersection_point);
        let t_coord = self.map(&normal);

        let hit_record = HitRecord::new(
            ray,
            normal,
            intersection_time,
            Some(self.material.clone()),
            Some(t_coord),
        );
        Some(hit_record)
    }
}

impl Shape for RSphere {
    fn bounding_box(&self) -> Option<AABB> {
        self.sphere.bounding_box()
    }
}

/// Renderable Sphere
pub struct RSphere {
    sphere: Sphere,
    material: Rc<Box<dyn Material>>,
}

impl RSphere {
    pub fn from_sphere(sphere: Sphere, material: Rc<Box<dyn Material>>) -> Self {
        RSphere { sphere, material }
    }
}

impl TextureMap for RSphere {
    /*
    Compute texture coordinates for a point on the surface of a unit sphere 
    */
    fn map(&self, p: &Point) -> TextureCoord {
        let theta = -p.y().acos();
        let phi = f64::atan2(-p.z(), p.x()) + PI;

        TextureCoord {
            u: phi / (2f64 * PI),
            v: theta / PI,
        }
    }
}

/*
Computes the intersection points between a given ray
and a sphere
*/
pub fn intersection(center: Point, radius: f64, ray: &Ray) -> Option<(f64, f64)> {
    let center_to_origin = ray.origin() - center;

    // quadratic formula
    let a = ray.direction().length_squared();

    if a == 0.0 {
        panic!("Error: division by zero");
    }

    let half_b = Vec3::dot(&center_to_origin, &ray.direction());
    let c = center_to_origin.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0f64 {
        return None;
    }

    let r1 = (-half_b - discriminant.sqrt()) / a;
    let r2 = (-half_b + discriminant.sqrt()) / a;

    Some((f64::min(r1, r2), f64::max(r1, r2)))
}



