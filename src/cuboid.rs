// use crate::{hittable::HitRecord, ray::Ray};
// use std::{cell::RefCell, rc::Rc};

// use crate::{hittable::Hittable, *};

// pub struct Triangle {
//     p0: Point,
//     p1: Point,
//     p2: Point,
//     normal: Vec3,
// }

// impl Triangle {
//     /// create a new triangle
//     /// points are assumed to be passed in CCW order
//     pub fn new(p0: Point, p1: Point, p2: Point) -> Triangle {
//         Triangle {
//             p0,
//             p1,
//             p2,
//             normal: Triangle::normal(p0, p1, p2),
//         }
//     }

//     /// compute distance from the origin (0, 0, 0) to the triangle's plane
//     fn compute_d(&self) -> f64 {
//         let normal = self.normal;

//         Vec3::dot(&normal, &self.p0)
//     }

//     /// compute the surface normal of the triangle
//     fn normal(p0: Point, p1: Point, p2: Point) -> Vec3 {
//         let edge_1: Vec3 = p1 - p0;
//         let edge_2: Vec3 = p2 - p0;

//         let normal = Vec3::normalized(Vec3::cross(&edge_1, &edge_2));

//         normal
//     }

//     fn compute_intersection_t(&self, ray: &Ray) -> f64 {
//         let normal = self.normal;
//         let d = self.compute_d();

//         let t = -(Vec3::dot(&normal, &ray.origin()) + d) / Vec3::dot(&normal, &ray.direction());

//         t
//     }
//     /// inside-outside test
//     /// TODO: create an edge_test function and then use that to perform the inside outside test
//     /// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-rendering-a-triangle/ray-triangle-intersection-geometric-solution
//     fn contains_point(&self, p: Point) -> bool {
//         let n = &self.normal;

//         let edge0 = self.p1 - self.p0;
//         let edge1 = self.p2 - self.p1;
//         let edge2 = self.p0 - self.p2;

//         let c0 = p - self.p0;
//         let c1 = p - self.p1;
//         let c2 = p - self.p2;

//         /*
//         point must must lie to the left of all vectors
//         parallel to the edges defining the triangle
//         */
//         if Vec3::dot(n, &Vec3::cross(&edge0, &c0)) > 0f64
//             && Vec3::dot(n, &Vec3::cross(&edge1, &c1)) > 0f64
//             && Vec3::dot(n, &Vec3::cross(&edge2, &c2)) > 0f64
//         {
//             true
//         } else {
//             false
//         }
//     }
// }

// impl Hittable for Triangle {
//     fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
//         let normal = self.normal;

//         if Vec3::dot(&normal, &ray.direction()) == 0f64 {
//             /*
//             the triangles plane and the ray are parallel and therefore
//             the ray and the plane will not intersect (unless the ray's
//             origin is on the plane; we ignore this case)
//             */
//             return None;
//         }

//         let t = self.compute_intersection_t(ray);

//         if t <= t_min {
//             /*
//             the ray has "past" the triangle and therefore they
//             do not intersect
//             */
//             return None;
//         }

//         let plane_intersection = ray.position_at(t);

//         // CLEAN: placeholder material
//         let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
//             0.7, 0.6, 0.5,
//         ))));

//         if self.contains_point(plane_intersection) {
//             let record = HitRecord::new(ray, normal, t, Some(m2));
//             Some(record)
//         } else {
//             None
//         }
//     }
// }

// pub struct Rectangle {
//     bottom_left: Point,
//     top_right: Point,
// }

// impl Rectangle {
//     pub fn new(bottom_left: Point, top_right: Point) -> Rectangle {
//         Rectangle {
//             bottom_left,
//             top_right,
//         }
//     }
// }

// impl Hittable for Rectangle {
//     fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
//         todo!()
//     }
// }

// pub struct Cuboid {
//     width: f64,  // x-axis
//     height: f64, // y-axis
//     depth: f64,  // z-axis
//     material: Rc<RefCell<Box<dyn Material>>>,
//     center: Point,
// }

// impl Cuboid {
//     pub fn new(
//         center: Point,
//         width: f64,
//         height: f64,
//         depth: f64,
//         material: Rc<RefCell<Box<dyn Material>>>,
//     ) -> Cuboid {
//         Cuboid {
//             width,
//             height,
//             depth,
//             material,
//             center,
//         }
//     }
// }

// impl Hittable for Cuboid {
//     fn hit(&self, ray: &ray::Ray, t_min: f64, t_max: f64) -> Option<hittable::HitRecord> {
//         /*
//         1. Construct planes out of the sides of the square.
//         2. Iterate over the planes and determine whether the ray intersects with any of them.
//         3. Record intersection times.
//         4.1 Case I  (Intersection): Using the minimum intersection time, calculate the reflection of the ray
//         4.2 Case II (No intersection): Return None
//         */
//         None
//     }
// }
