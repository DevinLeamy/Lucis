use crate::math::*;

pub fn write_color(pixel_color: Color) {
    println!(
        "{} {} {}",
        map_normalized_component(pixel_color[0]),
        map_normalized_component(pixel_color[1]),
        map_normalized_component(pixel_color[2]),
    )
}

fn map_normalized_component(c: f64) -> i32 {
    (c * 255.0).floor() as i32
}

pub type Color = Vec3;
