use serde::{Deserialize, Serialize};
use std::ops;

use crate::utils::random_float;

// pub use glam::Vec3;

#[derive(Debug, Copy, Clone, PartialEq, Default, Serialize, Deserialize)]
#[readonly::make]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn ones() -> Self {
        Self {
            x: 1f32,
            y: 1f32,
            z: 1f32,
        }
    }
    pub fn zeros() -> Self {
        Self {
            x: 0f32,
            y: 0f32,
            z: 0f32,
        }
    }

    pub fn random() -> Self {
        Self {
            x: random_float(),
            y: random_float(),
            z: random_float(),
        }
    }

    pub fn normalized(v: Vec3) -> Vec3 {
        v / v.length()
    }
    pub fn normalize(self) -> Vec3 {
        Vec3::normalized(self)
    }
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(lhs: Vec3, rhs: Vec3) -> f32 {
        lhs[0] * rhs[0] + lhs[1] * rhs[1] + lhs[2] * rhs[2]
    }

    pub fn cross(lhs: Vec3, rhs: Vec3) -> Vec3 {
        Vec3::new(
            lhs[1] * rhs[2] - lhs[2] * rhs[1],
            lhs[2] * rhs[0] - lhs[0] * rhs[2],
            lhs[0] * rhs[1] - lhs[1] * rhs[0],
        )
    }

    pub fn near_zero(&self) -> bool {
        let tolerance = 1e-8;

        self[0].abs() < tolerance && self[1].abs() < tolerance && self[2].abs() < tolerance
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Invalid vec3 index ({})", index),
        }
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Invalid vec3 index ({})", index),
        }
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self[0] /= rhs[0];
        self[1] /= rhs[1];
        self[2] /= rhs[2];
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f32) -> Self {
        if t == 0f32 {
            panic!("Division by zero");
        } else {
            Vec3::new(self.x / t, self.y / t, self.z / t)
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, t: f32) -> Self {
        Vec3::new(self.x * t, self.y * t, self.z * t)
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
