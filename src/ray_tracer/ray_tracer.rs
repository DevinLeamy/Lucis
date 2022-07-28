use std::{
    borrow::BorrowMut,
    cell::RefCell,
    io::{self, Write},
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
    time::Instant,
};

use crate::{
    common::{color::Color, random_float, INFINITY},
    hittable::Hittable,
    hittable_list::HittableList,
    math::Vec3,
    ray::Ray,
    scenes::{complex_not_random_scene, simple_scene},
};

use super::{camera::Camera, Frame};
pub struct RayTracer {}

impl RayTracer {
    pub const SAMPLES_PER_PIXEL: u32 = (50 / 1) as u32;
    const MAXIMUM_BOUNCE_DEPTH: u32 = 50;
    const THREAD_COUNT: u32 = 8;

    fn tile_render(
        frame: Arc<Mutex<Box<Frame>>>,
        camera: Camera,
        scene: HittableList,
        thread_id: u32,
    ) {
        let mut frame = frame.lock().unwrap();
        let image_width = frame.width();
        let image_height = frame.height();

        for j in (0..image_height).rev() {
            // eprintln!(
            //     "Progress: [{:.2}%] Time Elapsed: [{:.2}s]",
            //     ((image_height - j) as f32 / image_height as f32) * 100.0,
            //     now.elapsed().as_secs_f32()
            // );
            io::stderr().flush();

            for i in 0..image_width {
                let tile_id = j * image_height + image_width;
                if tile_id % RayTracer::THREAD_COUNT != thread_id {
                    continue;
                }

                let mut pixel_color = Color::ZEROS();

                for _ in 0..RayTracer::SAMPLES_PER_PIXEL {
                    let u = (i as f64 + random_float()) / ((image_width - 1) as f64); // pixel x coordinate
                    let v = (j as f64 + random_float()) / ((image_height - 1) as f64); // pixel y coordinate

                    let ray = camera.create_ray(u, v);
                    pixel_color += RayTracer::ray_color(&ray, &scene, 0);
                }

                let normalized_color =
                    RayTracer::normalize_color(pixel_color, RayTracer::SAMPLES_PER_PIXEL);

                frame.borrow_mut().set_color(i, j, normalized_color);
            }
        }
    }

    pub fn render(
        image_width: u32,
        image_height: u32,
        camera: Camera,
        scene: HittableList,
    ) -> Arc<Mutex<Box<Frame>>> {
        let frame = Arc::new(Mutex::new(Box::new(Frame::new(image_width, image_height))));
        let mut threads: Vec<JoinHandle<_>> = vec![];
        let now = Instant::now();

        for thread_id in 0..RayTracer::THREAD_COUNT {
            let frame_clone = Arc::clone(&frame);
            let scene_clone = scene.clone();
            threads.push(thread::spawn(move || {
                RayTracer::tile_render(frame_clone, camera, simple_scene(), thread_id);
            }));
        }

        for thread in threads {
            thread.join().unwrap();
        }

        eprintln!("Render complete [{:.2}s]", now.elapsed().as_secs_f32());

        frame
    }

    pub fn normalize_color(pixel_color: Color, pixel_samples: u32) -> Color {
        let scale = 1.0 / (pixel_samples as f64);
        let scaled_color = pixel_color * scale;

        let gamma2_corrected = Vec3::new(
            scaled_color[0].sqrt(),
            scaled_color[1].sqrt(),
            scaled_color[2].sqrt(),
        );

        let normalized = Vec3::new(
            RayTracer::map_normalized_component(gamma2_corrected[0]).into(),
            RayTracer::map_normalized_component(gamma2_corrected[1]).into(),
            RayTracer::map_normalized_component(gamma2_corrected[2]).into(),
        );

        normalized
    }

    fn map_normalized_component(c: f64) -> i32 {
        (c * 255.0).floor() as i32
    }

    fn ray_color(ray: &Ray, world: &HittableList, bounce_depth: u32) -> Color {
        if bounce_depth == RayTracer::MAXIMUM_BOUNCE_DEPTH {
            return Color::ZEROS();
        }

        // 0.001 used to avoid "shadow acne"
        if let Some(hit_record) = world.hit(ray, 0.01, INFINITY) {
            /*
            Compute a target point for the bounced ray by picking a random point inside
            a unit sphere tangent to point of intersection. Then determine
            the color obtained from the resulting bounced ray
            */
            if let Some((attenuation, bounced_ray)) = hit_record
                .clone()
                .material
                .unwrap()
                .borrow()
                .scatter(ray, &hit_record)
            {
                attenuation * RayTracer::ray_color(&bounced_ray, world, bounce_depth + 1)
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
}
