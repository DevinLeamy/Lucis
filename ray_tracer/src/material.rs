use crate::collisions::{CollisionRecord, Face};
use crate::image::Color;
use crate::ray::Ray;
use crate::texture::{Texture, TextureType};
use crate::utils::{random_unit_vector, reflect, sample_unit_sphere};
use crate::vec3::Vec3;

use serde::{Deserialize, Serialize};

pub struct CollisionResult {
    pub reflected_ray: Ray,
    pub color: Color,
    pub emitted_light: Color,
}

pub trait Material {
    /// Takes in a ray and a collision record and performs
    /// collision resolution
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult;
}

#[derive(Clone, Deserialize, Serialize)]
pub enum MaterialType {
    Lambertian(Lambertian),
    Dielectric(Dielectric),
    Metal(Metal),
    DiffuseLight(DiffuseLight),
}

impl Material for MaterialType {
    // This allows us to resolve collision for non-homogenous data
    // without having to use trait objects
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        match self {
            MaterialType::Dielectric(m) => m.resolve(ray, collision),
            MaterialType::Lambertian(m) => m.resolve(ray, collision),
            MaterialType::Metal(m) => m.resolve(ray, collision),
            MaterialType::DiffuseLight(m) => m.resolve(ray, collision),
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Lambertian {
    texture: TextureType,
}

/// Reflects all incoming rays with the same intensity.
/// Rays are reflected in random directions with a distribution
/// consistent with Lambertian reflection
impl Lambertian {
    pub fn new(texture: TextureType) -> Lambertian {
        Lambertian { texture }
    }
}

impl Material for Lambertian {
    fn resolve(&self, _ray: Ray, collision: CollisionRecord) -> CollisionResult {
        let mut bounce_dir = collision.normal() + random_unit_vector();

        if bounce_dir.near_zero() {
            bounce_dir = collision.normal();
        }

        CollisionResult {
            reflected_ray: Ray::new(collision.point, bounce_dir),
            color: self.texture.value(collision.uv, collision.point),
            emitted_light: Color::new(0.0, 0.0, 0.0),
        }
    }
}

/// Perfectly reflects/refracts all incoming rays. No light intensity is lost.
/// Rays are reflected or refracted based on the refractive index of the material
#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct Dielectric {
    ref_index: f64,
}

impl Dielectric {
    pub fn new(ref_index: f64) -> Dielectric {
        Dielectric { ref_index }
    }

    fn reflectance(cos: f64, ref_ratio: f64) -> f64 {
        // Schlick's approximation to reflectance
        let x = (1.0 - ref_ratio) / (1.0 + ref_ratio);
        x * x + (1.0 - x * x) * f64::powf(1.0 - cos, 5.0)
    }
}

impl Material for Dielectric {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        let ref_ratio = match collision.face {
            Face::Outer => 1.0 / self.ref_index,
            Face::Inner => self.ref_index,
        };

        let cos = f64::min(Vec3::dot(-ray.direction, collision.normal()), 1.0);
        let sin = (1.0 - cos * cos).sqrt();

        let must_reflect = ref_ratio * sin > 1.0;
        let veto_refract = Dielectric::reflectance(cos, ref_ratio) > crate::utils::random_float();

        let reflect = must_reflect || veto_refract;

        CollisionResult {
            reflected_ray: match reflect {
                true => ray.reflect(collision.normal(), collision.point),
                false => ray.refract(collision.normal(), collision.point, ref_ratio),
            },
            color: Color::new(1.0, 1.0, 1.0),
            emitted_light: Color::new(0.0, 0.0, 0.0),
        }
    }
}

/// Reflects all incoming rays with the same intensity.
/// Rays reflect perfectly off of the surface of the object.
/// Fuzz [0, 1] determines how much the perfectly reflected
/// rays are perturbed
#[derive(Clone, Deserialize, Serialize)]
pub struct Metal {
    texture: TextureType,
    fuzz: f64,
}

impl Metal {
    pub fn new(texture: TextureType, fuzz: f64) -> Self {
        Metal { texture, fuzz }
    }
}

impl Material for Metal {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        let ref_ray_dir = reflect(ray.direction, collision.normal());
        let ref_ray = Ray::new(
            collision.point,
            /*
            add fuzziness to reflection by perturbing the reflected
            ray by selecting a target point inside a sphere of radius (fuzz)
            centered at the non-perturbed rays target
            */
            ref_ray_dir + sample_unit_sphere() * self.fuzz,
        );

        match collision.face {
            Face::Outer => CollisionResult {
                color: self.texture.value(collision.uv, collision.point),
                reflected_ray: ref_ray,
                emitted_light: Color::new(0.0, 0.0, 0.0),
            },
            // ray cannot escape
            Face::Inner => CollisionResult {
                color: Color::black(),
                reflected_ray: ref_ray,
                emitted_light: Color::new(0.0, 0.0, 0.0),
            },
        }
    }
}

#[derive(Clone, Deserialize, Serialize)]
pub struct DiffuseLight {
    hue: Color,
    brightness: f64,
}

impl DiffuseLight {
    pub fn new(hue: Color, brightness: f64) -> DiffuseLight {
        DiffuseLight { hue, brightness }
    }
}

impl Material for DiffuseLight {
    fn resolve(&self, ray: Ray, collision: CollisionRecord) -> CollisionResult {
        /*
        This is a temporary hack. Basically, currently we have "color" determined
        seperately from "emitted_light". Eventually these two things will
        be combined by applying the Blinn-Phong reflection model

        For now, to determine the light coming from light emitting surfaces,
        we will simply zero out the impact of all incoming light on the surface
        by making the attentuation "color"=Color(0, 0, 0).
        */
        CollisionResult {
            reflected_ray: ray,
            color: Color::new(0.0, 0.0, 0.0),
            emitted_light: self.hue * self.brightness,
        }
    }
}
