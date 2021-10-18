use std::fs::File;
use std::io::Write;

use anyhow::Result;

use raytracer::Color3;

fn write_color(mut file: &File, color: Color3) -> Result<()> {
    let r = (color.x * 255.999) as i32;
    let g = (color.y * 255.999) as i32;
    let b = (color.z * 255.999) as i32;
    writeln!(file, "{} {} {}", r, g, b)?;
    Ok(())
}

fn main() -> Result<()> {
    // Image settings
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;
    const MAX_COLOR: i32 = 255;

    // Display PPM file
    let name = "image.ppm";
    let mut file = File::create(name).expect("Failed to create file!");

    // Write header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "{}", MAX_COLOR)?;

    // Write data
    for j in (0..IMAGE_HEIGHT).rev() {
        print!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let pixel = Color3::new(
                i as f32 / IMAGE_WIDTH as f32,
                j as f32 / IMAGE_HEIGHT as f32,
                0.25,
            );
            write_color(&file, pixel)?;
        }
    }
    println!("\nDone!");
    Ok(())
}
