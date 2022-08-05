pub use rand::*;
pub use std::cell::RefCell;
use std::ops::Add;
use std::ops::Sub;
pub use std::rc::Rc;

pub use crate::components::*;
pub use crate::hittable::*;
pub use crate::hittable_list::*;
pub use crate::math::*;
pub use crate::ray::*;
pub use crate::ray_tracer::*;
pub use crate::sphere::*;
pub use crate::utils::*;
pub use std::f64::consts::PI;

pub const INFINITY: f64 = f64::MAX;

pub fn random_float() -> f64 {
    thread_rng().gen()
}

pub fn random_float_in_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_float()
}

pub fn random_natural(min: u32, max: u32) -> u32 {
    ((min + (max - min)) as f64 * random_float()) as u32
}

pub struct Interval {
    /// start of an interval
    t0: f64,
    /// end of an interval
    t1: f64,
}

impl Interval {
    pub fn new(start: f64, end: f64) -> Interval {
        assert!(start <= end);
        Interval { t0: start, t1: end }
    }

    // pub fn overlap(i1: Interval, i2: Interval) -> bool {}
}
pub struct IntervalV3 {
    /// start of an interval
    t0: Vec3,
    /// end of an interval
    t1: Vec3,
}

impl IntervalV3 {
    pub fn IntervalV3(start: Vec3, end: Vec3) -> IntervalV3 {
        assert!(start[0] <= end[0] && start[1] <= end[1] && start[2] <= end[2]);
        IntervalV3 { t0: start, t1: end }
    }
}
