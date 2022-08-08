use crate::perlin::*;
use crate::core::*;

pub struct PerlinTexture {
    noise_gen: Perlin
}

impl PerlinTexture {
    pub fn new() -> PerlinTexture {
        PerlinTexture {
            noise_gen: Perlin::new()
        }
    }
}

impl Texture for PerlinTexture {
    fn value(&self, uv: &TextureCoord, point: &Point) -> Color {
        Vec3::ONES() * self.noise_gen.noise(point)
    }
}

