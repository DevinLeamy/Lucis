use crate::{PerlinTexture, Camera, CameraConfig, DiffuseLight};
use crate::collisions::{Collidable, CollisionRecord};
use crate::image::Color;
use crate::material::{Material, MaterialType, Dielectric, Lambertian, Metal};
use crate::ray::Ray;
use crate::shape::{ShapeType, Sphere, RectangleXY, RectangleXZ, RectangleYZ, Box};
use crate::texture::{TextureType, CheckeredTexture};
use crate::utils::random_float;
use crate::vec3::Vec3;
use serde::{Deserialize, Serialize};


#[readonly::make]
#[derive(Clone, Deserialize, Serialize)]
pub struct Element {
    pub id: ElementId,
    pub material: MaterialType, 
    pub shape: ShapeType,
}

impl Element {
    pub fn new(material: MaterialType, shape: ShapeType) -> Element {
        Element {
            id: ElementId::new(),
            material,
            shape
        }
    }
    pub fn set_material(&mut self, material: MaterialType) {
        self.material = material;
    }
}

impl Collidable for Element {
    fn collide(&self, ray: Ray) -> Option<CollisionRecord> {
        self.shape.collide(ray)
    }
}

#[readonly::make]
#[derive(Copy, Clone, PartialEq, Debug, Serialize, Deserialize)]
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
#[derive(Clone)]
pub struct Scene {
    pub objects: Vec<Element> 
}

impl Default for Scene {
    fn default() -> Scene { Scene::one_sphere() }
}

impl Scene {
    /// remove element by id
    pub fn remove(&mut self, id: ElementId) {
        self.objects.retain(|e| e.id != id);
    }

    /// get element by id
    pub fn get_element_mut(&mut self, id: ElementId) -> &mut Element {
        self.objects
            .iter_mut()
            .find(|e| e.id == id)
            .expect("ElementId not found")
    }

    pub fn get_element(&self, id: ElementId) -> &Element {
        self.objects
            .iter()
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

    pub fn one_sphere() -> Scene {
        Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Metal(Metal::new(Color::new(0.2, 0.2, 0.9).into(), 0.2)),
                    // material: MaterialType::Lambertian(Lambertian::new(PerlinTexture::new_scaled(4.0).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(-1.0, 0.5, -1.0), 0.5))
                },
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(
                        TextureType::CheckeredTexture(CheckeredTexture::new(
                            Color::white(),
                            Color::new(0.8, 0.8, 1.0),
                        )))
                    ),
                    // material: MaterialType::Lambertian(Lambertian::new(PerlinTexture::new().into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0))
                },
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

    // TODO: make sure that the element is centered at (0, 0)
    pub fn sphere(element: Element) -> Scene {
        Scene {
            objects: vec![
                element,
                // ground
                Element {
                    id: ElementId::new(),
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.7, 0.7, 0.7).into())),
                    // material: MaterialType::Lambertian(Lambertian::new(TextureType::PerlinTexture(PerlinTexture::new()))),
                    material: MaterialType::Lambertian(Lambertian::new(
                        TextureType::CheckeredTexture(CheckeredTexture::new(
                            Color::white(), 
                            Color::new(0.1, 0.1, 0.1),
                        )))
                    ),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.5, 0.0), 1000.0))
                }, 
            ]
        }
   }

   pub fn element_with_background(element: Element, background_mat: MaterialType) -> Scene {
        Scene {
            objects: vec![
                element,
                Element {
                    id: ElementId::new(),
                    material: background_mat, 
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.5, 0.0), 1000.0))
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
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Metal(Metal::new(Color::new(0.6, 0.2, 0.0).into(), 0.0)),
                    // material: MaterialType::Lambertian(Lambertian::new(PerlinTexture::new().into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(2.0, 0.5, -1.0), 0.5))
                },
            ]
        }
    }

    pub fn rectangles() -> (Camera, Scene) {
        let camera = Camera::new(
            CameraConfig {
                origin: Vec3::new(-4.0, 3.0, 4.0),
                look_at: Vec3::new(0.5, 0.5, 0.0),
                aspect: 1.0,
                ..CameraConfig::default()
            }
        );
        
        let scene = Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Metal(Metal::new(Color::new(8.0, 1.0, 8.0).into(), 0.0)),
                    // material: MaterialType::Lambertian(Lambertian::new(PerlinTexture::new().into())),
                    // material: MaterialType::DiffuseLight(DiffuseLight::new(Color::white(), 15.0)),
                    shape: ShapeType::Box(Box::cube(0.5, Vec3::new(0.2, 1.0, 0.0)))
                },
                Element {
                    id: ElementId::new(),
                    material: MaterialType::Lambertian(Lambertian::new(Color::new(0.2, 0.8, 0.2).into())),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(1.0, 0.5, -1.0), 0.5))
                },
                // ground
                Element {
                    id: ElementId::new(),
                    // material: MaterialType::Lambertian(Lambertian::new(Color::new(0.1, 0.1, 0.1).into())),
                    // material: MaterialType::DiffuseLight(DiffuseLight::new(Color::white(), 15.0)),
                material: MaterialType::Lambertian(Lambertian::new(
                        TextureType::CheckeredTexture(CheckeredTexture::new(
                            Color::white(),
                            Color::new(0.1, 0.1, 0.1),
                        )))
                    ),
                    shape: ShapeType::Sphere(Sphere::new(Vec3::new(0.0, -1000.5, 0.0), 1000.0))
                }, 
            ]
        };

        return (camera, scene)
   } 

    pub fn cornell_box() -> (Camera, Scene) {
        let camera = Camera::new(
            CameraConfig {
                origin: Vec3::new(278.0, 278.0, -800.0),
                look_at: Vec3::new(278.0, 278.0, 0.0),
                aspect: 1.0,
                vertical_fov_degrees: 40.0,
                ..CameraConfig::default()
            }
        );

        let red = MaterialType::Lambertian(Lambertian::new(Color::new(0.65, 0.05, 0.05).into()));
        let white = MaterialType::Lambertian(Lambertian::new(Color::new(0.73, 0.73, 0.73).into()));
        let green = MaterialType::Lambertian(Lambertian::new(Color::new(0.12, 0.45, 0.15).into()));
        let light = MaterialType::DiffuseLight(DiffuseLight::new(Color::new(1.0, 1.0, 1.0), 15.0));
        
        let scene = Scene {
            objects: vec![
                Element {
                    id: ElementId::new(),
                    material: green.clone(), 
                    shape: ShapeType::RectangleYZ(RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 555.0, true))
                },
                Element {
                    id: ElementId::new(),
                    material: red.clone(), 
                    shape: ShapeType::RectangleYZ(RectangleYZ::new(0.0, 555.0, 0.0, 555.0, 0.0, false))
                },
                Element {
                    id: ElementId::new(),
                    material: light.clone(), 
                    shape: ShapeType::RectangleXZ(RectangleXZ::new(213.0, 343.0, 227.0, 332.0, 554.0, true))
                },
                Element {
                    id: ElementId::new(),
                    material: white.clone(), 
                    shape: ShapeType::RectangleXZ(RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 0.0, false))
                }, 
                Element {
                    id: ElementId::new(),
                    material: white.clone(), 
                    shape: ShapeType::RectangleXZ(RectangleXZ::new(0.0, 555.0, 0.0, 555.0, 555.0, true))
                }, 
                Element {
                    id: ElementId::new(),
                    material: white.clone(), 
                    shape: ShapeType::RectangleXY(RectangleXY::new(0.0, 555.0, 0.0, 555.0, 555.0, false))
                }, 
                Element {
                    id: ElementId::new(),
                    material: white.clone(),
                    shape: ShapeType::Box(Box::new(Vec3::new(130.0, 0.0, 65.0), Vec3::new(295.0, 165.0, 230.0)))
                },
                Element {
                    id: ElementId::new(),
                    material: white.clone(),
                    shape: ShapeType::Box(Box::new(Vec3::new(265.0, 0.0, 295.0), Vec3::new(430.0, 330.0, 460.0)))
                }
            ]
        };

        (camera, scene)
    } 
}
