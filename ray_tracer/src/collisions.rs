use crate::{ray::Ray, shape::UV, vec3::Vec3, ElementId};

#[derive(Debug, PartialEq)]
pub enum Face {
    Outer,
    Inner,
}
pub struct CollisionRecord {
    pub point: Vec3,
    /// surface normal at intersection point
    pub s_normal: Vec3,
    pub t: f64,
    pub uv: UV,
    pub face: Face,
}

impl CollisionRecord {
    /// normal opposing the ray's direction
    pub fn normal(&self) -> Vec3 {
        match self.face {
            Face::Outer => self.s_normal,
            Face::Inner => -self.s_normal,
        } 
    }
}

pub trait Collidable {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord>; 
}

pub fn collision_face(incident: Vec3, normal: Vec3) -> Face {
    if Vec3::dot(incident, normal) < 0.0 { Face::Outer }
    else { Face::Inner }
}
