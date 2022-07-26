use crate::common::Color;

pub struct Frame {
    width: u32,
    height: u32,
    buffer: Vec<Vec<Color>>,
}

impl Frame {
    pub fn new(frame_width: u32, frame_height: u32) -> Frame {
        Frame {
            width: frame_width,
            height: frame_height,
            buffer: vec![vec![Color::ZEROS(); frame_width as usize]; frame_height as usize],
        }
    }

    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        self.buffer[y as usize][x as usize] = color;
    }

    pub fn write_to_console(&self) {
        for j in (0..self.height as usize).rev() {
            for i in 0..self.width as usize {
                println!(
                    "{} {} {}",
                    self.buffer[j][i][0], self.buffer[j][i][1], self.buffer[j][i][2]
                );
            }
        }
    }
}
