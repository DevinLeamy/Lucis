use crate::vec3::Vec3;

pub struct Image {
    width: u32,
    height: u32,
    /// buffer[i][j] => row(i) col(j)
    /// buffer[0][0] = top left
    buffer: Vec<Vec<Color>>,
}

impl Image {
    pub fn new(frame_width: u32, frame_height: u32) -> Image {
        Image {
            width: frame_width,
            height: frame_height,
            buffer: vec![vec![Color::default(); frame_width as usize]; frame_height as usize],
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        self.buffer[y as usize][x as usize] = color;
    }

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        self.buffer[y as usize][x as usize]
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn clear(&mut self) {
        for color in self.buffer.iter_mut().flatten() {
            *color = Color::default();
        }
   }
}

trait WritePPM {
    fn write_as_ppm(&self) -> ();
}

impl WritePPM for Image {
    fn write_as_ppm(&self) -> () {
        println!("P3\n{} {}\n255", self.width, self.height);
        for color in self.buffer.iter().flatten() {
            Color::display_as_u8(*color);
        }
    }
}

#[derive(Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64 
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn to_u8(channel: f64) -> u8 {
        (channel * 255f64) as u8
    }

    pub fn display_as_u8(color: Color) {
        println!(
            "{} {} {}",
            Color::to_u8(color.red),
            Color::to_u8(color.green),
            Color::to_u8(color.blue),
        ) 
    }

    pub fn white() -> Color { Color::new(1.0, 1.0, 1.0) }
    pub fn black() -> Color { Color::new(0.0, 0.0, 0.0) }
}

impl Default for Color {
    fn default() -> Color { 
        Color { red: 0f64, green: 0f64, blue: 0f64 }
    }
}

impl From<Vec3> for Color {
    fn from(v: Vec3) -> Self {
        Color::new(v[0], v[1], v[2])
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, rhs: Color) -> Color {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

