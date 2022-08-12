use crate::vec3::Vec3;
use crate::aabb::{AABB, Boundable};

pub struct UV(f64, f64);

trait TextureMap {
    fn map(&self, point: Vec3) -> UV;
}

trait SurfaceNormal {
    fn surface_normal(&self, point: Vec3) -> Vec3;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64) -> Sphere {
        Sphere { center, radius }
    }

    pub fn radius(&self) -> f64 { self.radius }
    pub fn center(&self) -> Vec3 { self.center }
}

impl TextureMap for Sphere {
    fn map(&self, point: Vec3) -> UV {
        todo!()
    }
}

impl SurfaceNormal for Sphere {
    fn surface_normal(&self, point: Vec3) -> Vec3 {
        (point - self.center).normalize()
    }
}

impl Boundable for Sphere {
    fn bound(&self) -> AABB {
        let min = self.center - Vec3::ones() * self.radius;
        let max = self.center + Vec3::ones() * self.radius;

        AABB::new(min, max)
    }
}
