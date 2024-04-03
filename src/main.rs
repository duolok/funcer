mod vec3;
use crate::vec3::Vec3;
use std::io::{self, Write, Result};

pub type Color = Vec3;

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

pub fn write_color(color: Vec3) {
    let r = (255.999 * color.x()) as i32;
    let g = (255.999 * color.y()) as i32;
    let b = (255.999 * color.z()) as i32;

    println!("{} {} {}", r, g, b);
}

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    
    for col in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - col);
        for row in 0..IMAGE_WIDTH {
            let color = Color::new((row as f32 / (IMAGE_WIDTH - 1) as f32).into(),
                               (col as f32 / (IMAGE_HEIGHT - 1) as f32).into(),
                               0.0);
            write_color(color);

        }
    }

    Ok(())
}
