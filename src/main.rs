use common::*;
use hittable_list::HittableList;
use ray_tracer::*;
use sphere::*;

mod common;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod ray_tracer;
mod sphere;
mod utils;

fn complex_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(
        Color::new(0.5, 0.5, 0.5),
    )));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    for i in -11..11 {
        for j in -11..11 {
            let mat = random_float();
            let origin = Point::new(
                i as f64 + 0.9 * random_float(),
                0.2,
                j as f64 * random_float(),
            );

            if (origin - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let material = if mat < 0.8 {
                    let albedo = Vec3::RAND() * Vec3::RAND();
                    make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(albedo)))
                } else if mat < 0.95 {
                    let albedo = Color::RAND_RANGE(0.5, 1.0);
                    let fuzz = random_float_in_range(0.0, 0.5);

                    make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(albedo, fuzz)))
                } else {
                    make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)))
                };

                world.add(make_shared_hittable(Box::new(Sphere::new(
                    origin, 0.2, material,
                ))))
            }
        }
    }

    let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
    let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));

    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m1,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m2,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m3,
    ))));

    world
}

pub fn simple_scene() -> HittableList {
    let mut world = HittableList::default();

    let ground_material = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(
        Color::new(0.5, 0.5, 0.5),
    )));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ))));

    let m1 = make_shared_material::<Box<dyn Material>>(Box::new(Dielectric::new(1.5)));
    let m2 = make_shared_material::<Box<dyn Material>>(Box::new(Lambertian::new(Color::new(
        0.4, 0.2, 0.1,
    ))));
    let m3 = make_shared_material::<Box<dyn Material>>(Box::new(Metal::new(
        Color::new(0.7, 0.6, 0.5),
        0.0,
    )));

    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m1,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m2,
    ))));
    world.add(make_shared_hittable(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m3,
    ))));

    world
}

fn main() {
    let aspect_ratio = 3.0 / 2.0;

    let camera_origin = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let world_up = Vec3::new(0.0, 1.0, 0.0);
    let distance_to_focus = 10.0;
    let aperture = 0.1;

    let camera: Camera = Camera::new(
        camera_origin,
        look_at,
        world_up,
        20.0,
        aspect_ratio,
        aperture,
        distance_to_focus,
    );

    let image_width: u32 = 200;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio()) as u32;

    println!("P3\n{} {}\n255", image_width, image_height);

    let frame = RayTracer::render(image_width, image_height, camera, simple_scene());
    frame.write_to_console();
}
