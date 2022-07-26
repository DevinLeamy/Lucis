use std::{
    borrow::Borrow,
    cell::RefCell,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

use common::*;
use hittable_list::HittableList;
use ray_tracer::*;
use scenes::*;
use sphere::*;

mod common;
mod hittable;
mod hittable_list;
mod math;
mod ray;
mod ray_tracer;
mod scenes;
mod sphere;
mod utils;

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

    let mut threads: Vec<JoinHandle<_>> = vec![];
    let thread_count = 8;

    let now = Instant::now();

    let frame = Arc::new(Mutex::new(Box::new(Frame::new(image_width, image_height))));

    for i in 0..thread_count {
        let clone = Arc::clone(&frame);
        threads.push(thread::spawn(move || {
            // let frame = RayTracer::render(image_width, image_height, camera, simple_scene());
            RayTracer::render(
                image_width,
                image_height,
                camera,
                complex_not_random_scene(),
                i,
                clone,
            );
            eprintln!("Thread [{}] complete", i);
        }));
    }

    // let mut frames: Vec<Frame> = vec![];

    for thread in threads {
        thread.join().unwrap();
    }

    // for i in 0..image_width {
    //     for j in 0..image_height {
    //         let mut color = Color::ZEROS();

    //         for frame in &frames {
    //             color += frame.get_color(i, j);
    //         }

    //         final_frame.set_color(
    //             i,
    //             j,
    //             RayTracer::normalize_color(color, RayTracer::SAMPLES_PER_PIXEL * thread_count),
    //         );
    //     }
    // }

    eprintln!("Render complete [{:.2}s]", now.elapsed().as_secs_f32());

    frame.lock().unwrap().borrow().write_to_console();
}
/*
-- Simple Scene (200px width, Aspect [16/9], 50 SAMPLES, 50 MAX_BOUNCE_DEPTH)
Threads: Time (s)
1:       [10.43, 10.52]
5:       [11.12]
8:       [11.72, 11.81]
9:       [13.35]
10:      [14.21]
11:      [15.14]
20:      [26.96] (and it slows down the computer)

-- Complex Scene (200px width, Aspect [16/9], 50 SAMPLES, 50 MAX_BOUNCE_DEPTH)
Threads: Time (s)
1:       [213.11]
8:       [237.16]
*/
