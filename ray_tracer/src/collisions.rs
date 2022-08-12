use crate::{ray::Ray, shape::UV, vec3::Vec3};

#[readonly::make]
pub struct CollisionRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub uv: UV
}

enum CollisionOutcome {
    NoCollision,
    Collision(CollisionRecord)
}

trait Collidable {
    fn collide(&self, ray: &Ray) -> CollisionOutcome; 
}
