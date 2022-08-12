use std::f64::consts::PI;

use crate::collisions::{CollisionRecord, Collidable};
use crate::vec3::Vec3;
use crate::aabb::{AABB, Boundable};
use crate::ray::Ray;

#[derive(Copy, Clone)]
pub struct UV {
    u: f64,
    v: f64
}

pub trait TextureMap {
    fn map(&self, point: Vec3) -> UV;
}

pub trait SurfaceNormal {
    fn surface_normal(&self, point: Vec3) -> Vec3;
}

#[derive(Clone, Copy)]
pub enum ShapeType {
    Sphere(Sphere)
}


#[derive(Copy, Clone)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn radius(&self) -> f64 { self.radius }
    pub fn center(&self) -> Vec3 { self.center }

    pub fn intersections(&self, ray: Ray) -> Option<(f64, f64)> {
        let center_to_origin = ray.origin - self.center;

        // quadratic formula
        let a = ray.direction.length_squared();

        if a == 0.0 {
            panic!("Error: division by zero");
        }

        let half_b = Vec3::dot(center_to_origin, ray.direction);
        let c = center_to_origin.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0f64 {
            return None;
        }

        let r1 = (-half_b - discriminant.sqrt()) / a;
        let r2 = (-half_b + discriminant.sqrt()) / a;

        Some((f64::min(r1, r2), f64::max(r1, r2)))
    }
}

impl TextureMap for Sphere {
    fn map(&self, p: Vec3) -> UV {
        let theta = (-p.y).acos();
        let phi = f64::atan2(-p.z, p.x) + PI;

        UV {
            u: phi / (2f64 * PI),
            v: theta / PI,
        }
    }
}

impl SurfaceNormal for Sphere {
    fn surface_normal(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}

impl Collidable for Sphere {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        let intersection_time = match self.intersections(ray) {
            Some((r1, r2)) => {
                f64::max(r1, r2)
                // if interval.contains(r1) { r1 } 
                // else if interval.contains(r2) { r2 } 
                // else { return None; }
            }
            _ => return None,
        };

        let intersection_point = ray.position_at(intersection_time);
        let normal = self.surface_normal(intersection_point);

        Some(CollisionRecord {
            point: ray.position_at(intersection_time),
            normal,
            t: intersection_time,
            uv: self.map(normal)
        })
    }
}

impl Boundable for Sphere {
    fn bound(&self) -> AABB {
        let min = self.center - Vec3::ones() * self.radius;
        let max = self.center + Vec3::ones() * self.radius;

        AABB::new(min, max)
    }
}
