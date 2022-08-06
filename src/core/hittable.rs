use crate::core::*;

#[derive(Clone, Default)]
pub struct HitRecord {
    /// point of intersection
    point: Point,
    /// surface normal of the intersected surface (always opposes the direction of the ray)
    normal: Vec3,
    /// time of the ray's intersection
    t: f64,
    /// material of the intersected surface
    material: Option<Rc<Box<dyn Material>>>,
    /// did the ray hit the outer face of the surface?
    hit_front_face: bool,
    /// texture coordinates
    t_coord: Option<TextureCoord>,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        outward_normal: Vec3,
        t: f64,
        material: Option<Rc<Box<dyn Material>>>,
        texture_coord: Option<TextureCoord>,
    ) -> HitRecord {
        let hit_front_face = Vec3::dot(&ray.direction(), &outward_normal) < 0.0;

        HitRecord {
            point: ray.position_at(t),
            t,
            material,
            hit_front_face,
            normal: if hit_front_face {
                outward_normal.clone()
            } else {
                -outward_normal.clone()
            },
            t_coord: texture_coord,
        }
    }

    pub fn material(&self) -> Option<Rc<Box<dyn Material>>> {
        self.material.clone()
    }

    pub fn point(&self) -> Point {
        self.point
    }

    pub fn normal(&self) -> Vec3 {
        self.normal
    }

    pub fn hit_front_face(&self) -> bool {
        self.hit_front_face
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn uv(&self) -> TextureCoord {
        self.t_coord.unwrap()
    }
}

// pub trait Hittable<T: Boundable> {
//     /// determine whether the hittable was hit by a ray during a time interval
//     fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
// }
