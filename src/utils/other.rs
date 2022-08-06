use std::borrow::Borrow;
pub use std::f64::consts::PI;
pub use crate::core::*;
pub use rand::*;


pub const INFINITY: f64 = f64::MAX;
// pub const PI: f64 = std::math::PI; 

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
    pub min: f64,
    pub max: f64,
}

impl Interval {
    pub fn contains(&self, time: f64) -> bool {
        self.min <= time && time <= self.max
    }

    pub fn ceil(&mut self, max: f64) {
        self.max = f64::min(self.max, f64::max(self.min, max));
    }

    pub fn floor(&mut self, min: f64) {
        self.min = f64::max(self.min, f64::min(self.max, min));
    }

    pub fn min(&self) -> f64 { self.min }
    pub fn max(&self) -> f64 { self.max }
}

pub struct SharedMut<T> {
    inner: Rc<RefCell<Box<T>>>
}

impl<T> SharedMut<T> {
    pub fn new(inner: T) -> SharedMut<T> {
        SharedMut {
            inner: Rc::new(RefCell::new(Box::new(inner)))
        }
    }

    pub fn as_ref(&self) -> &RefCell<Box<T>> {
        &self.inner.as_ref()
    }

    pub fn as_mut(&mut self) -> &RefCell<Box<T>> {
        self.inner.as_ref()
    }
}

pub struct Shared<T> {
    inner: Rc<Box<T>>
}

impl<T> Shared<T> {
    pub fn new(inner: T) -> Shared<T> {
        Shared {
            inner: Rc::new(Box::new(inner))
        }
    }

    pub fn as_ref(&self) -> &T {
        &self.inner.as_ref().borrow()
    }
}
