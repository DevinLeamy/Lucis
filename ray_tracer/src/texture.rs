use std::sync::Arc;

use crate::image::Color;
use crate::perlin::Perlin;
use crate::shape::UV;
use crate::vec3::Vec3;

use serde::{Deserialize, Serialize};

pub trait Texture {
    fn value(&self, uv: UV, point: Vec3) -> Color;
}

#[derive(Clone, Deserialize, Serialize)]
pub enum TextureType {
    CheckeredTexture(CheckeredTexture),
    SolidTexture(SolidTexture),
    PerlinTexture(PerlinTexture),
}

impl Texture for TextureType {
    fn value(&self, uv: UV, point: Vec3) -> Color {
        match self {
            TextureType::CheckeredTexture(tx) => tx.value(uv, point),
            TextureType::SolidTexture(tx) => tx.value(uv, point),
            TextureType::PerlinTexture(tx) => tx.value(uv, point),
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct CheckeredTexture {
    odd: Color,
    even: Color,
}

impl CheckeredTexture {
    pub fn new(odd: Color, even: Color) -> Self {
        CheckeredTexture { odd, even }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, _uv: UV, p: Vec3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 {
            self.odd
        } else {
            self.even
        }
    }
}

#[derive(Copy, Clone, Serialize, Deserialize)]
pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> Self {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _uv: UV, _p: Vec3) -> Color {
        self.color
    }
}

impl From<Color> for TextureType {
    fn from(color: Color) -> Self {
        TextureType::SolidTexture(SolidTexture::new(color))
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(from = "DeserializePerlinTexture")]
pub struct PerlinTexture {
    scale: f32,
    #[serde(skip)]
    noise_gen: Arc<Box<Perlin>>,
}

#[derive(Deserialize)]
pub struct DeserializePerlinTexture {
    scale: f32,
}

impl From<DeserializePerlinTexture> for PerlinTexture {
    fn from(serialized: DeserializePerlinTexture) -> PerlinTexture {
        PerlinTexture::new_scaled(serialized.scale)
    }
}

impl PerlinTexture {
    pub fn new() -> PerlinTexture {
        PerlinTexture::new_scaled(1.0)
    }

    pub fn new_scaled(scale: f32) -> PerlinTexture {
        PerlinTexture {
            noise_gen: Arc::new(Box::new(Perlin::new())),
            scale,
        }
    }
}

impl Texture for PerlinTexture {
    // fn value(&self, _uv: UV, point: Vec3) -> Color {
    //     // TODO: make depth configurable
    //     let depth = 7;

    //     Color::new(
    //       0.5 * self.noise_gen.turbulence(point * self.scale, depth),
    //       0.5 * self.noise_gen.turbulence(point * self.scale, depth),
    //       0.5 * self.noise_gen.turbulence(point * self.scale, depth),
    //     )
    // }
    fn value(&self, _uv: UV, point: Vec3) -> Color {
        // TODO: make depth configurable
        let depth = 7;

        let noise_manip = |noise: f32| 0.5 * (1.0 + f32::sin(self.scale * point.z + 10.0 * noise));

        Color::new(
            noise_manip(self.noise_gen.turbulence(point, depth)),
            noise_manip(self.noise_gen.turbulence(point, depth)),
            noise_manip(self.noise_gen.turbulence(point, depth)),
        )
    }
}

impl From<PerlinTexture> for TextureType {
    fn from(perlin: PerlinTexture) -> Self {
        TextureType::PerlinTexture(perlin)
    }
}
