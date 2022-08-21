use std::f64::consts::PI;

use crate::collisions::{CollisionRecord, Collidable, collision_face};
use crate::vec3::Vec3;
use crate::aabb::{AABB, Boundable};
use crate::ray::Ray;

use serde::{Serialize, Deserialize};

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

#[derive(Clone, Serialize, Deserialize)]
pub enum ShapeType {
    Sphere(Sphere),
    RectangleXY(RectangleXY),
    RectangleXZ(RectangleXZ),
    RectangleYZ(RectangleYZ),
    Box(Box),
}

impl Collidable for ShapeType {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
       match self {
            ShapeType::Sphere(c)      => c.collide(ray),
            ShapeType::RectangleXY(c) => c.collide(ray),
            ShapeType::RectangleXZ(c) => c.collide(ray),
            ShapeType::RectangleYZ(c) => c.collide(ray),
            ShapeType::Box(c)         => c.collide(ray)
       } 
    }
}


#[derive(Clone, Serialize, Deserialize)]
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
            Some((r1, r2)) => f64::min(r1, r2),
            _ => return None,
        };

        let intersection_point = ray.position_at(intersection_time);
        let s_normal = self.surface_normal(intersection_point);

        Some(CollisionRecord {
            point: intersection_point, 
            s_normal,
            t: intersection_time,
            uv: self.map(s_normal),
            face: collision_face(ray.direction, s_normal),
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

/// Axis-aligned rectangle
#[derive(Clone, Serialize, Deserialize)]
pub struct RectangleXY {
    /// center of the rectangle 
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
    flip_normal: bool,
}

impl RectangleXY {
    const SURFACE_NORMAL: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, flip_normal: bool) -> RectangleXY {
        RectangleXY { x0, x1, y0, y1, k, flip_normal }
    }
}

impl Collidable for RectangleXY {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        let t = (self.k - ray.origin.z) / ray.direction.z;
        let x = ray.origin.x + ray.direction.x * t;
        let y = ray.origin.y + ray.direction.y * t;

        if self.x0 <= x && x <= self.x1 && self.y0 <= y && y <= self.y1 {
            let c_point = Vec3::new(x, y, self.k);
            let s_normal = self.surface_normal(c_point); 

            Some(CollisionRecord {
                point: c_point, 
                t,
                s_normal,
                uv: self.map(c_point),
                face: collision_face(ray.direction, s_normal)
            })
        } else {
            None
        }
    }
}

impl Boundable for RectangleXY {
    fn bound(&self) -> AABB {
        let min = Vec3::new(self.x0, self.y0, self.k - 0.001);
        let max = Vec3::new(self.x1, self.y1, self.k + 0.001);

        AABB::new(min, max)
    }
}

impl SurfaceNormal for RectangleXY {
    fn surface_normal(&self, point: Vec3) -> Vec3 {
        RectangleXY::SURFACE_NORMAL * (if self.flip_normal { -1.0 } else { 1.0 })
    }
}

impl TextureMap for RectangleXY {
    fn map(&self, point: Vec3) -> UV {
        UV { 
            u: (point.x - self.x0) / (self.x1 - self.x0), 
            v: (point.y - self.y0) / (self.y1 - self.y0) 
        }
    }
}


/// Axis-aligned rectangle
#[derive(Clone, Serialize, Deserialize)]
pub struct RectangleXZ {
    /// center of the rectangle 
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    flip_normal: bool,
}

impl RectangleXZ {
    const SURFACE_NORMAL: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, flip_normal: bool) -> RectangleXZ {
        RectangleXZ { x0, x1, z0, z1, k, flip_normal }
    }
}

impl Collidable for RectangleXZ {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        let t = (self.k - ray.origin.y) / ray.direction.y;
        let x = ray.origin.x + ray.direction.x * t;
        let z = ray.origin.z + ray.direction.z * t;

        if self.x0 <= x && x <= self.x1 && self.z0 <= z && z <= self.z1 {
            let c_point = Vec3::new(x, self.k, z);
            let s_normal = self.surface_normal(c_point); 

            Some(CollisionRecord {
                point: c_point, 
                t,
                s_normal,
                uv: self.map(c_point),
                face: collision_face(ray.direction, s_normal)
            })
        } else {
            None
        }
    }
}

impl Boundable for RectangleXZ {
    fn bound(&self) -> AABB {
        let min = Vec3::new(self.x0, self.k - 0.001, self.z0);
        let max = Vec3::new(self.x1, self.k + 0.001, self.z1);

        AABB::new(min, max)
    }
}

impl SurfaceNormal for RectangleXZ {
    fn surface_normal(&self, point: Vec3) -> Vec3 {
        RectangleXZ::SURFACE_NORMAL * (if self.flip_normal { -1.0 } else { 1.0 })
    }
}

impl TextureMap for RectangleXZ {
    fn map(&self, point: Vec3) -> UV {
        UV { 
            u: (point.x - self.x0) / (self.x1 - self.x0), 
            v: (point.z - self.z0) / (self.z1 - self.z0) 
        }
    }
}

/// Axis-aligned rectangle
#[derive(Clone, Serialize, Deserialize)]
pub struct RectangleYZ {
    /// center of the rectangle 
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    flip_normal: bool
}

impl RectangleYZ {
    const SURFACE_NORMAL: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, flip_normal: bool) -> RectangleYZ {
        RectangleYZ { y0, y1, z0, z1, k, flip_normal }
    }
}

impl Collidable for RectangleYZ {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        let t = (self.k - ray.origin.x) / ray.direction.x;
        let y = ray.origin.y + ray.direction.y * t;
        let z = ray.origin.z + ray.direction.z * t;

        if self.y0 <= y && y <= self.y1 && self.z0 <= z && z <= self.z1 {
            let c_point = Vec3::new(self.k, y, z);
            let s_normal = self.surface_normal(c_point); 

            Some(CollisionRecord {
                point: c_point, 
                t,
                s_normal,
                uv: self.map(c_point),
                face: collision_face(ray.direction, s_normal)
            })
        } else {
            None
        }
    }
}

impl Boundable for RectangleYZ {
    fn bound(&self) -> AABB {
        let min = Vec3::new(self.k - 0.001, self.y0, self.z0); 
        let max = Vec3::new(self.k + 0.001, self.y1, self.z1);

        AABB::new(min, max)
    }
}

impl SurfaceNormal for RectangleYZ {
    fn surface_normal(&self, point: Vec3) -> Vec3 {
        RectangleYZ::SURFACE_NORMAL * (if self.flip_normal { -1.0 } else { 1.0 })
    }
}

impl TextureMap for RectangleYZ {
    fn map(&self, point: Vec3) -> UV {
        UV { 
            u: (point.y - self.y0) / (self.y1 - self.y0), 
            v: (point.z - self.z0) / (self.z1 - self.z0) 
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum RectangleType {
    RectangleXY(RectangleXY),
    RectangleYZ(RectangleYZ),
    RectangleXZ(RectangleXZ),
}

impl Collidable for RectangleType {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        match self {
            RectangleType::RectangleXY(r) => r.collide(ray),
            RectangleType::RectangleXZ(r) => r.collide(ray),
            RectangleType::RectangleYZ(r) => r.collide(ray),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Box {
    min: Vec3,
    max: Vec3,
    sides: Vec<RectangleType>
}

impl Box {
    pub fn new(min: Vec3, max: Vec3) -> Box {
        Box {
            min,
            max,
            sides: vec![
                // back and front
                RectangleType::RectangleXY(RectangleXY::new(min.x, max.x, min.y, max.y, min.z, true)),
                RectangleType::RectangleXY(RectangleXY::new(min.x, max.x, min.y, max.y, max.z, false)),
                // top and bottom
                RectangleType::RectangleXZ(RectangleXZ::new(min.x, max.x, min.z, max.z, min.y, true)),
                RectangleType::RectangleXZ(RectangleXZ::new(min.x, max.x, min.z, max.z, max.y, false)),
                // left and right
                RectangleType::RectangleYZ(RectangleYZ::new(min.y, max.y, min.z, max.z, min.x, true)),
                RectangleType::RectangleYZ(RectangleYZ::new(min.y, max.y, min.z, max.z, max.x, false)),

            ]
        }
    }

    pub fn from_size(width: f64, height: f64, depth: f64, center: Vec3) -> Box {
        let half_w = width / 2.0;
        let half_h = height / 2.0;
        let half_d = depth / 2.0; 

        Box::new(
            Vec3::new(center.x - half_w, center.y - half_h, center.z - half_d),
            Vec3::new(center.x + half_w, center.y + half_h, center.z + half_d),
        )
    }

    pub fn cube(side_length: f64, center: Vec3) -> Box {
        Box::from_size(side_length, side_length, side_length, center)
    }
}

impl Collidable for Box {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        let mut c_record: Option<CollisionRecord> = None;

        for side in &self.sides {
            if let Some(record) = side.collide(ray) {
                if c_record.as_ref().is_none() || record.t < c_record.as_ref().unwrap().t {
                    c_record = Some(record); 
                }
            }
        }

        c_record
    }
}

impl Boundable for Box {
    fn bound(&self) -> AABB {
        AABB::new(self.min, self.max)
    }
}
