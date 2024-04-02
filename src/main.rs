use std::io::{self, Write, Result};

const IMAGE_WIDTH: u16 = 256;
const IMAGE_HEIGHT: u16 = 256;

fn main() -> Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    
    for col in 0..IMAGE_HEIGHT {
        eprintln!("\rScanlines remaining: {} ", IMAGE_HEIGHT - col);
        for row in 0..IMAGE_WIDTH {
            let r = row as f32 / (IMAGE_WIDTH - 1) as f32;
            let g = col as f32 / (IMAGE_HEIGHT - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            writeln!(handle, "{} {} {}", ir, ig, ib)?;
        }
    }

    Ok(())
}
