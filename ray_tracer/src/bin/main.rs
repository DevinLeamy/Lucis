use ray_tracer::*;

fn main() {
    let width = 800u32;
    let height = width;

    let (camera, scene) = Scene::rectangles();

    let image =
        RayTracer::new(RayTracerConfig::default()).render_scene(&scene, camera, width, height);

    image.write_as_ppm();
}
