use crate::camera::Camera;
use crate::collisions::{CollisionRecord, Collidable};
use crate::material::{MaterialType, Material, CollisionResult};
use crate::image::{Color, Image};
use crate::ray::Ray;
use crate::scene::{Scene, Element};
use crate::shape::{ShapeType, SurfaceNormal, TextureMap};
use crate::vec3::Vec3;

const MAX_BOUNCE_DEPTH: u32 = 50;

trait Render {
    fn render_scene(scene: &Scene, camera: Camera, width: u32, height: u32) -> Image; 
}

struct RayTracer {
}

impl RayTracer {
    fn compute_ray_color(scene: &Scene, ray: Ray, bounce_depth: u32) -> Color {
        if bounce_depth == MAX_BOUNCE_DEPTH {
            return Color::black();
        }

        let mut c_record: Option<CollisionRecord> = None;
        let mut c_t = f64::MAX;
        let mut c_material: Option<MaterialType> = None; 

        scene.objects.iter().for_each(|element| {
            let shape = element.shape;

            let collidable = match shape {
                ShapeType::Sphere(sphere) => sphere
            };
            
            if let Some(record) = collidable.collide(ray) {
                // update the collision record if 
                // the ray collides earlier
                if 0f64 < record.t && record.t < c_t { 
                    let c_point = ray.position_at(c_t);
                    c_t = record.t;
                    c_material = Some(element.material);
                    c_record = Some(CollisionRecord {
                        point: c_point, 
                        normal: collidable.surface_normal(c_point),
                        t: record.t,
                        uv: collidable.map(c_point),
                    })
                } 
            };
        });

        if c_material.is_none() {
            return Color::new(0.5, 0.8, 0.2);
        }

        let result = match c_material.unwrap() {
            MaterialType::Dielectric(m) => m.resolve(ray, c_record.unwrap()),
            MaterialType::Lambertian(m) => m.resolve(ray, c_record.unwrap()),
        };

        result.color * RayTracer::compute_ray_color(scene, result.reflected_ray, bounce_depth + 1)
    }
}

impl Render for RayTracer {
    fn render_scene(scene: &Scene, camera: Camera, width: u32, height: u32) -> Image {
        let mut image = Image::new(width, height); 

        let pixels = width * height;

        for i in 0..pixels {
            let row = i / width;
            let col = i % width;

            let ray = camera.create_ray(row as f64, col as f64);

            let color = RayTracer::compute_ray_color(scene, ray, 0); 

            image.set_color(row, col, color);
        }

        image
    }
}

