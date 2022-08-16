use crate::image::Color;
use crate::material::{Material, MaterialType, Dielectric, Lambertian, Metal};
use crate::shape::{ShapeType, Sphere};
use crate::texture::{TextureType, CheckeredTexture};
use crate::utils::random_float;
use crate::vec3::Vec3;


#[readonly::make]
#[derive(Clone)]
pub struct Element {
    pub id: ElementId,
    pub material: MaterialType, 
    pub shape: ShapeType,
}

#[readonly::make]
#[derive(Copy, Clone, PartialEq)]
pub struct ElementId {
    pub id: u64
}

impl ElementId {
    pub fn new() -> ElementId {
        ElementId {
            id: (random_float() * (u64::MAX - 1) as f64) as u64
        }
    }
}

#[readonly::make]
pub struct Scene {
    pub objects: Vec<Element> 
}

impl Scene {
    /// remove element by id
    pub fn remove(&mut self, id: ElementId) {
        self.objects.retain(|e| e.id != id);
    }

    /// get element by id
    pub fn get(&mut self, id: ElementId) -> &mut Element {
        self.objects
            .iter_mut()
            .find(|e| e.id == id)
            .expect("ElementId not found")
    }
}

impl Scene {
    pub fn two_spheres() -> Scene {
        Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.7, 0.2, 0.5).into())),
                    // material: MaterialType::Dielectric(Dielectric::new(0.5)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -5.0, 0.0), 5.0))
                },
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 5.0, 0.0), 5.0))
                }
            ]
        }
    }

    pub fn simple() -> Scene {
        Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.2, 0.2).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))
                },
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0))
                },
                Element {
                    id: ElementId::new(),
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.6, 0.8, 0.6))),
                    material: MaterialType::Dielectric(Dielectric::new(1.52)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0))
                }, 
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.9).into(), 0.2)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0))
                },
            ]
        }
    }

    pub fn materials() -> Scene {
        Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(
                        TextureType::CheckeredTexture(CheckeredTexture::new(
                            Color::white(),
                            Color::new(0.8, 0.8, 1.0),
                        )))
                    ),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))
                },
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(-1.0, 0.5, -1.0), 0.5))
                },
                Element {
                    id: ElementId::new(),
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.6, 0.8, 0.6))),
                    material: MaterialType::Dielectric(Dielectric::new(1.52)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, 0.5, -1.0), 0.5))
                }, 
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.6).into(), 0.0)),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(1.0, 0.5, -1.0), 0.5))
                },
            ]
        }
    }
}
