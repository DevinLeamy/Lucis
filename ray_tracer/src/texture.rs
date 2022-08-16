use std::sync::Arc;

use crate::shape::UV;
use crate::image::Color;
use crate::vec3::Vec3;
use crate::perlin::Perlin;

pub trait Texture {
    fn value(&self, uv: UV, point: Vec3) -> Color;
}

#[derive(Clone)]
pub enum TextureType {
    CheckeredTexture(CheckeredTexture),
    SolidTexture(SolidTexture),
    PerlinTexture(PerlinTexture)
}

impl Texture for TextureType {
    fn value(&self, uv: UV, point: Vec3) -> Color {
        match self {
            TextureType::CheckeredTexture(tx) => tx.value(uv, point),
            TextureType::SolidTexture(tx)     => tx.value(uv, point),
            TextureType::PerlinTexture(tx)    => tx.value(uv, point),
        }
    }
}

#[derive(Copy, Clone)]
pub struct CheckeredTexture {
    odd: Color,
    even: Color
}

impl CheckeredTexture {
    pub fn new(odd: Color, even: Color) -> Self {
        CheckeredTexture { odd, even }
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, _uv: UV, p: Vec3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();

        if sines < 0.0 { self.odd } 
        else { self.even }
    }
}

#[derive(Copy, Clone)]
pub struct SolidTexture {
    color: Color
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

#[derive(Clone)]
pub struct PerlinTexture {
    noise_gen: Arc<Box<Perlin>>
}

impl PerlinTexture {
    pub fn new() -> PerlinTexture {
        PerlinTexture { noise_gen: Arc::new(Box::new(Perlin::new())) }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, _uv: UV, point: Vec3) -> Color {
        Color::new(
          self.noise_gen.smooth_noise(point),   
          self.noise_gen.smooth_noise(point),   
          self.noise_gen.smooth_noise(point),   
        )
    }
}

impl From<PerlinTexture> for TextureType {
    fn from(perlin: PerlinTexture) -> Self {
        TextureType::PerlinTexture(perlin)
    }
}


