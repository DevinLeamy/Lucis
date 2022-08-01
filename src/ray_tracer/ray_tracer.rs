use crate::{
    common::{color::Color, random_float, INFINITY},
    hittable::Hittable,
    hittable_list::HittableList,
    math::Vec3,
    ray::Ray,
};

use super::{camera::Camera, Frame};
pub struct RayTracer {
    config: RayTracerConfig,
    camera: Camera,
    frame: Frame,
}

pub struct RayTracerConfig {
    samples_per_pixel: u32,
    maximum_bounce_depth: u32,
    thread_count: u32,
}

impl RayTracerConfig {
    pub fn default() -> RayTracerConfig {
        RayTracerConfig {
            samples_per_pixel: 8,
            maximum_bounce_depth: 50,
            thread_count: 1,
        }
    }
}

impl RayTracer {
    pub fn new(config: RayTracerConfig, camera: Camera, frame: Frame) -> RayTracer {
        RayTracer {
            config,
            camera,
            frame,
        }
    }

    pub fn set_camera(&mut self, camera: Camera) {
        self.camera = camera;
    }

    pub fn set_frame(&mut self, frame: Frame) {
        self.frame = frame;
    }

    pub fn set_config(&mut self, config: RayTracerConfig) {
        self.config = config;
    }

    fn tile_render(&mut self, scene: &HittableList, thread_id: u32) {
        let image_width = self.frame.width();
        let image_height = self.frame.height();

        for j in (0..image_height).rev() {
            for i in 0..image_width {
                let tile_id = j * image_height + image_width;
                if tile_id % self.config.thread_count != thread_id {
                    continue;
                }
                self.color_pixel(i, j, scene);
            }
        }
    }

    fn frame_to_world(&self, x: f64, y: f64) -> (f64, f64) {
        (
            x / (self.frame.width() - 1) as f64,
            y / (self.frame.height() - 1) as f64,
        )
    }

    fn color_pixel(&mut self, x: u32, y: u32, scene: &HittableList) {
        let pixel_color = (0..self.config.samples_per_pixel)
            .map(|_| {
                let (u, v) =
                    self.frame_to_world(x as f64 + random_float(), y as f64 + random_float());
                let ray = self.camera.create_ray(u, v);
                self.ray_color(&ray, &scene, 0)
            })
            .fold(Color::ZEROS(), |color, new_color| color + new_color);

        self.frame
            .set_color(x, y, self.normalize_color(pixel_color));
    }

    pub fn render(&mut self, scene: &HittableList) -> Frame {
        self.frame.clear();

        for thread_id in 0..self.config.thread_count {
            self.tile_render(scene, thread_id);
        }

        eprintln!("Render complete!");

        self.frame.clone()
    }

    pub fn normalize_color(&self, pixel_color: Color) -> Color {
        let scale = 1.0 / (self.config.samples_per_pixel as f64);
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

    fn ray_color(&self, ray: &Ray, world: &HittableList, bounce_depth: u32) -> Color {
        if bounce_depth == self.config.maximum_bounce_depth {
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
                .material()
                .unwrap()
                .borrow()
                .scatter(ray, &hit_record)
            {
                attenuation * self.ray_color(&bounced_ray, world, bounce_depth + 1)
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
