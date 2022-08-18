use ray_tracer::*;

fn main() {
    let width = 400u32;
    let height = width; 

    let (camera, scene) = Scene::rectangles(); 

    let image = RayTracer::render_scene(
        &scene,
        camera,
        width,
        height
    );

    image.write_as_ppm();
}
