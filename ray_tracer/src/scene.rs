use crate::image::Color;
use crate::material::{Material, MaterialType, Dielectric, Lambertian, Metal};
use crate::shape::{ShapeType, Sphere};
use crate::vec3::Vec3;

#[readonly::make]
#[derive(Copy, Clone)]
pub struct Element {
    pub material: MaterialType, 
    pub shape: ShapeType,
}

#[readonly::make]
pub struct Scene {
    pub objects: Vec<Element> 
}

impl Scene {
    pub fn two_spheres() -> Scene {
        Scene {
            objects: vec![
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.7, 0.2, 0.5))),
                    // material: MaterialType::Dielectric(Dielectric::new(0.5)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -5.0, 0.0), 5.0))
                },
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2))),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 5.0, 0.0), 5.0))
                }
            ]
        }
    }

    pub fn simple() -> Scene {
        Scene {
            objects: vec![
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.2, 0.2))),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))
                },
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2))),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0))
                },
                Element {
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.6, 0.8, 0.6))),
                    material: MaterialType::Dielectric(Dielectric::new(1.52)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0))
                }, 
                Element {
                    material: MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.9), 0.2)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0))
                },
            ]
        }
    }

    pub fn materials() -> Scene {
        Scene {
            objects: vec![
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.2, 0.2))),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))
                },
                Element {
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2))),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(-1.0, 0.5, -1.0), 0.5))
                },
                Element {
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.6, 0.8, 0.6))),
                    material: MaterialType::Dielectric(Dielectric::new(1.52)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 0.5, -1.0), 0.5))
                }, 
                Element {
                    material: MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.6), 0.0)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(1.0, 0.5, -1.0), 0.5))
                },
            ]
        }
    }
}
