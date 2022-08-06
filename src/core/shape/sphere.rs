use crate::core::*;

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Self {
        Sphere { center, radius }
    }

    pub fn surface_normal(&self, point: Point) -> Vec3 {
        Vec3::normalized(point - self.center)
    }

    pub fn radius(&self) -> f64 { self.radius }
    pub fn center(&self) -> Point { self.center }
}

impl Shape for Sphere {
    fn bounding_box(&self) -> Option<AABB> {
        let center = self.center();
        let radius = self.radius();

        let min = center - Vec3::ONES() * radius;
        let max = center + Vec3::ONES() * radius;
        Some(AABB::new(min, max))
    }
}
