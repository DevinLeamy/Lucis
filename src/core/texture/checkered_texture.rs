use crate::core::*;

pub struct CheckeredTexture {
    odd: Rc<Box<dyn Texture>>,
    even: Rc<Box<dyn Texture>>,
}

impl CheckeredTexture {
    pub fn new(odd: Rc<Box<dyn Texture>>, even: Rc<Box<dyn Texture>>) -> CheckeredTexture {
        CheckeredTexture {
            odd: Rc::clone(&odd),
            even: Rc::clone(&even),
        }
    }

    pub fn from_colors(odd: &Color, even: &Color) -> CheckeredTexture {
        CheckeredTexture::new(
            Rc::new(Box::new(SolidTexture::new(*odd))),
            Rc::new(Box::new(SolidTexture::new(*even))),
        )
    }
}

impl Texture for CheckeredTexture {
    fn value(&self, uv: &TextureCoord, p: &Point) -> Color {
        let sines = (10f64 * p.x()).sin() * (10f64 * p.y()).sin() * (10f64 * p.z()).sin();

        if sines < 0.0 {
            self.odd.as_ref().as_ref().value(uv, p)
        } else {
            self.even.as_ref().as_ref().value(uv, p)
        }
    }
}
