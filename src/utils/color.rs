use crate::math::*;

pub fn write_color(pixel_color: Color, pixel_samples: u32) {
    let scale = 1.0 / (pixel_samples as f64);
    let scaled_color = pixel_color * scale;

    let gamma2_corrected = Vec3::new(
        scaled_color[0].sqrt(),
        scaled_color[1].sqrt(),
        scaled_color[2].sqrt(),
    );

    println!(
        "{} {} {}",
        map_normalized_component(gamma2_corrected[0]),
        map_normalized_component(gamma2_corrected[1]),
        map_normalized_component(gamma2_corrected[2]),
    )
}

fn map_normalized_component(c: f64) -> i32 {
    (c * 255.0).floor() as i32
}

pub type Color = Vec3;
