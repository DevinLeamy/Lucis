use crate::common::Color;

#[derive(Clone)]
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

    pub fn get_color(&self, x: u32, y: u32) -> Color {
        self.buffer[y as usize][x as usize]
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn clear(&mut self) {
        for row in self.buffer.iter_mut() {
            for color in row.iter_mut() {
                *color = Color::ZEROS();
            }
        }
    }

    pub fn write_to_console(&self) {
        println!("P3\n{} {}\n255", self.width, self.height);
        for j in (0..self.height as usize).rev() {
            for i in 0..self.width as usize {
                println!(
                    "{} {} {}",
                    self.buffer[j][i][0], self.buffer[j][i][1], self.buffer[j][i][2]
                );
            }
        }
    }

    // pub fn display(&self, window: &mut Window) {
    //     window.clear();
    //     for i in 0..self.width as usize {
    //         for j in 0..self.height as usize {
    //             let color = self.buffer[j][i];

    //             window.set_color(color[0] as u8, color[1] as u8, color[2] as u8, 1);
    //             window.draw_point(Point::new(i as i32, j as i32));
    //         }
    //     }
    // }
}
