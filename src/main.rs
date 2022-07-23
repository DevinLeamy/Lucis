use common::*;
use hittable::{HitRecord, Hittable};
use hittable_list::HittableList;
use ray::*;
use ray_tracer::*;
use sphere::*;
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
    if world.hit(ray, 0.001, INFINITY, &mut hit_record) {
        /*
        Compute a target point for the bounced ray by picking a random point inside
        a unit sphere tangent to point of intersection. Then determine
        the color obtained from the resulting bounced ray
        */
        // let target: Point = hit_record.point + hit_record.normal + random_unit_vector();
        let target: Point = hit_record.point + sample_hemisphere(&hit_record.normal);
        let bounced_ray = Ray::new(hit_record.point, target - hit_record.point);
        ray_color(&bounced_ray, world, bounce_depth + 1) * 0.5
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
    let camera: Camera = Camera::default();
    let image_width: u32 = 400;
    let image_height: u32 = (image_width as f32 / camera.aspect_ratio()) as u32;

    println!("P3\n{} {}\n255", image_width, image_height);

    let mut world = HittableList::default();
    world.add(Rc::new(RefCell::new(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
    )))));
    world.add(Rc::new(RefCell::new(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
    )))));

    for j in (0..image_height).rev() {
        eprintln!("Progress: [{}/{}]", image_height - j, image_height);
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

    eprintln!("Render complete");
}
