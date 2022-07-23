use math::*;
use ray::*;
use std::io::{self, Write};
use utils::*;

mod math;
mod ray;
mod utils;

const ASPECT_RATIO: f32 = 16.0 / 9.0;

const VIEWPORT_HEIGHT: f32 = 2.0;
const VIEWPORT_WIDTH: f32 = VIEWPORT_HEIGHT * ASPECT_RATIO;
const FOCAL_LENGTH: f32 = 1.0;

const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as u32;

const CAMERA_ORIGIN: Point = Point::ZEROS();
const CAMERA_HORIZONTAL: Vec3 = Vec3::new(VIEWPORT_WIDTH as f64, 0.0, 0.0);
const CAMERA_VERTICAL: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT as f64, 0.0);

// compute a simple gradient
fn ray_color(ray: &Ray) -> Color {
    let direction = Vec3::normalized(ray.direction());
    let t = 0.5 * (direction.y() + 1.0);

    Color::ONES() * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn main() {
    let CAMERA_LOWER_LEFT: Vec3 = CAMERA_ORIGIN
        - CAMERA_HORIZONTAL / 2.0
        - CAMERA_VERTICAL / 2.0
        - Vec3::new(0.0, 0.0, FOCAL_LENGTH as f64);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Progress: [{}/{}]", IMAGE_HEIGHT - j, IMAGE_HEIGHT);
        io::stderr().flush();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(
                CAMERA_ORIGIN,
                CAMERA_LOWER_LEFT + CAMERA_HORIZONTAL * u + CAMERA_VERTICAL * v - CAMERA_ORIGIN,
            );

            write_color(ray_color(&ray));
        }
        println!("");
    }

    eprintln!("Render complete");
}
