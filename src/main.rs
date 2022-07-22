use math::*;
use std::io::{self, Write};

mod math;

const IMAGE_WIDTH: u32 = 256;
const IMAGE_HEIGHT: u32 = 256;

fn main() {
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for i in 0..IMAGE_HEIGHT {
        eprintln!("Progress: [{}/{}]", i, IMAGE_HEIGHT);
        io::stderr().flush();

        for j in 0..IMAGE_WIDTH {
            let r = (i as f32) / ((IMAGE_HEIGHT - 1) as f32);
            let g = (j as f32) / ((IMAGE_WIDTH - 1) as f32);
            let b = 0.25f32;

            let int_r = (r * 255.0).floor() as u32;
            let int_g = (g * 255.0).floor() as u32;
            let int_b = (b * 255.0).floor() as u32;

            print!("{} {} {}  ", int_r, int_g, int_b);
        }
        println!("");
    }

    eprintln!("Render complete");
}
