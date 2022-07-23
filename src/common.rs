use rand::*;

pub use crate::math::*;
pub use crate::utils::*;
pub use std::f64::consts::PI;

pub const INFINITY: f64 = f64::MAX;

pub fn random_float() -> f64 {
    thread_rng().gen()
}

pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}
