use std::fmt::Display;

use crate::vec3::Vec3;

pub struct Image {
    height: u32,
    width: u32,
    /// buffer[i][j] => row(i) col(j)
    /// buffer[0][0] = top left
    buffer: Vec<Vec<ColorU8>>,
}

impl Image {
    pub fn new(rows: u32, cols: u32) -> Image {
        Image {
            height: rows,
            width: cols,
            buffer: vec![vec![ColorU8::black(); cols as usize]; rows as usize],
        }
    }

    pub fn set_color(&mut self, row: u32, col: u32, color: ColorU8) {
        self.buffer[row as usize][col as usize] = color;
    }

    pub fn get_color(&self, row: u32, col: u32) -> ColorU8 {
        self.buffer[row as usize][col as usize]
    }

    pub fn width(&self) -> u32 { self.width }
    pub fn height(&self) -> u32 { self.height }

    pub fn clear(&mut self) {
        for color in self.buffer.iter_mut().flatten() {
            *color = ColorU8::black();
        }
   }
}

pub trait WritePPM {
    fn write_as_ppm(&self) -> ();
}

impl WritePPM for Image {
    fn write_as_ppm(&self) -> () {
        println!("P3\n{} {}\n255", self.width, self.height);

        for i in (0..self.height).rev() {
            for j in 0..self.width {
                println!("{}", self.get_color(i, j));
            }
        }
    }
}

#[readonly::make]
#[derive(Copy, Clone)]
pub struct ColorU8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorU8 {
    pub fn black() -> ColorU8 { ColorU8 { red: 0, blue: 0, green: 0 } }
}

impl Display for ColorU8 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{} {} {}", self.red, self.green, self.blue))
    }
}

impl From<Color> for ColorU8 {
    fn from(color: Color) -> Self {
        ColorU8 {
            red: Color::to_u8(color.red),
            green: Color::to_u8(color.green),
            blue: Color::to_u8(color.blue),
        }
    }
} 

#[derive(Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64 
}


impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color { red, green, blue }
    }

    pub fn to_u8(channel: f64) -> u8 {
        (channel * 255f64) as u8
    }

    pub fn gamma_corrected(&self) -> Color {
        Color {
            red: self.red.sqrt(),
            blue: self.blue.sqrt(),
            green: self.green.sqrt(),
        } 
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

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl std::ops::Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self {
        Color::new(self.red + rhs.red, self.green + rhs.green, self.blue + rhs.blue)
    }
}

