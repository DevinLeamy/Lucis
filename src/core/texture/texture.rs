use crate::core::*;

pub trait Texture {
    fn value(&self, uv: &TextureCoord, point: &Point) -> Color;
}

#[derive(Clone, Copy)]
pub struct TextureCoord {
    pub u: f64,
    pub v: f64,
}

impl Default for TextureCoord {
    fn default() -> TextureCoord {
        TextureCoord { u: 0f64, v: 0f64 }
    }
}

pub trait TextureMap {
    fn map(&self, point: &Point) -> TextureCoord;
}

pub struct SolidTexture {
    color: Color,
}

impl SolidTexture {
    pub fn new(color: Color) -> SolidTexture {
        SolidTexture { color }
    }
}

impl Texture for SolidTexture {
    fn value(&self, _uv: &TextureCoord, _p: &Point) -> Color {
        self.color
    }
}
