use common::*;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::*;
use ray_tracer::*;
use sphere::*;
use std::time::Instant;
use std::{
    cell::RefCell,
    io::{self, Write},
    rc::Rc,
};

mod common;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod ray_tracer;
mod sphere;
mod utils;

// const SAMPLES_PER_PIXEL: u32 = 500;
const SAMPLES_PER_PIXEL: u32 = 10;
const MAXIMUM_BOUNCE_DEPTH: u32 = 50;

fn ray_color(ray: &Ray, world: &HittableList, bounce_depth: u32) -> Color {
    if bounce_depth == MAXIMUM_BOUNCE_DEPTH {
        return Color::ZEROS();
    }

    let mut hit_record = HitRecord::default();

    // 0.001 used to avoid "shadow acne"
    if world.hit(ray, 0.01, INFINITY, &mut hit_record) {
        /*
        Compute a target point for the bounced ray by picking a random point inside
        a unit sphere tangent to point of intersection. Then determine
        the color obtained from the resulting bounced ray
        */
        let mut bounced_ray = Ray::default();
        let mut attenuation = Color::default();

        let hr_clone = hit_record.clone();

        if hit_record.material.unwrap().borrow().scatter(
            ray,
            &hr_clone,
            &mut attenuation,
            &mut bounced_ray,
        ) {
            // println!(
            //     "Ray Origin: {:?} \nRay Direction: {:?} \nResult Origin: {:?} \nResult Direction: {:?}",
            //     ray.origin(),
            //     ray.direction(),
            //     // attenuation,
            //     bounced_ray.origin(),
            //     bounced_ray.direction(), // ray_color(&bounced_ray, world, bounce_depth + 1)
            // );
            attenuation * ray_color(&bounced_ray, world, bounce_depth + 1)
        } else {
            Color::ZEROS()
        }
    } else {
        let direction = Vec3::normalized(ray.direction());
        let t = 0.5 * (direction.y() + 1.0);

        // compute a simple gradient
        // blended_value = (1 - t) * start_value t * end_value
        Color::ONES() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        // ^^^white                 ^^^blue
    }
}

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

    for i in 0..11 {
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

fn main() {
    let now = Instant::now();
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

    // let image_width: u32 = 1200;
    let image_width: u32 = 200;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio()) as u32;

    println!("P3\n{} {}\n255", image_width, image_height);

    let world = complex_scene();

    for j in (0..image_height).rev() {
        eprintln!(
            "Progress: [{:.2}%] Time Elapsed: [{:.2}s]",
            ((image_height - j) as f32 / image_height as f32) * 100.0,
            now.elapsed().as_secs_f32()
        );
        io::stderr().flush();

        for i in 0..image_width {
            let mut pixel_color = Color::ZEROS();

            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_float()) / ((image_width - 1) as f64); // pixel x coordinate
                let v = (j as f64 + random_float()) / ((image_height - 1) as f64); // pixel y coordinate

                let ray = camera.create_ray(u, v);
                pixel_color += ray_color(&ray, &world, 0);
            }

            write_color(pixel_color, SAMPLES_PER_PIXEL);
        }
        println!("");
    }

    eprintln!("Render complete [{:.2}s]", now.elapsed().as_secs_f32());
}
