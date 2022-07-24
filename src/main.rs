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

const SAMPLES_PER_PIXEL: u32 = 100;
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

fn main() {
    let now = Instant::now();
    let aspect_ratio = 16.0 / 9.0;

    let camera: Camera = Camera::new(90.0, aspect_ratio);
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f64 / camera.aspect_ratio()) as u32;

    let R = (PI / 4.0).cos();

    println!("P3\n{} {}\n255", image_width, image_height);

    type MaterialPointer = Rc<RefCell<Box<dyn Material>>>;

    let material_left: MaterialPointer = Rc::new(RefCell::new(Box::new(Lambertian::new(
        Color::new(0.0, 0.0, 1.0),
    ))));
    let material_right: MaterialPointer = Rc::new(RefCell::new(Box::new(Lambertian::new(
        Color::new(1.0, 0.0, 0.0),
    ))));

    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Box::new(Sphere::new(
        Point::new(-R, 0.0, -1.0),
        R,
        material_left,
    )))));
    world.add(Rc::new(RefCell::new(Box::new(Sphere::new(
        Point::new(R, 0.0, -1.0),
        R,
        material_right,
    )))));

    for j in (0..image_height).rev() {
        eprintln!(
            "Progress: [{}/{}] Time Elapsed: [{:.2}s]",
            image_height - j,
            image_height,
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
