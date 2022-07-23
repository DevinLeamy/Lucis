pub use vec3::*;

pub mod vec3;

pub fn clamp(min: f64, max: f64, value: f64) -> f64 {
    f64::min(max, f64::max(min, value))
}
