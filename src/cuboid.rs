use std::rc::Rc;

use crate::{hittable::Hittable, *};

pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}

impl Hittable for Triangle {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        None
    }
}

pub struct Rectangle {
    bottom_left: Point,
    top_right: Point,
}

impl Rectangle {
    pub fn new(bottom_left: Point, top_right: Point) -> Rectangle {
        Rectangle {
            bottom_left,
            top_right,
        }
    }
}

impl Hittable for Rectangle {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        todo!()
    }
}

pub struct Cuboid {
    width: f64,  // x-axis
    height: f64, // y-axis
    depth: f64,  // z-axis
    material: Rc<RefCell<Box<dyn Material>>>,
    center: Point,
}

impl Cuboid {
    pub fn new(
        center: Point,
        width: f64,
        height: f64,
        depth: f64,
        material: Rc<RefCell<Box<dyn Material>>>,
    ) -> Cuboid {
        Cuboid {
            width,
            height,
            depth,
            material,
            center,
        }
    }
}

impl Hittable for Cuboid {
    fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
        /*
        1. Construct planes out of the sides of the square.
        2. Iterate over the planes and determine whether the ray intersects with any of them.
        3. Record intersection times.
        4.1 Case I  (Intersection): Using the minimum intersection time, calculate the reflection of the ray
        4.2 Case II (No intersection): Return None
        */
        None
    }
}
