use ray_tracer::*;

fn main() {
    let scene = Scene::materials();   
    let image_width = 600;
    
    let image = RayTracer::render_scene(&scene, Camera::default(), image_width, (image_width as f64 * 9.0 / 16.0) as u32); 

    image.write_as_ppm();
}

