use std::fs::File;
use std::io::Write;

use anyhow::Result;

use raytracer::{Color, Point, Ray, Vec3};

fn write_color(mut file: &File, color: &Color) -> Result<()> {
    let r = (color.x() * 255.999) as i32;
    let g = (color.y() * 255.999) as i32;
    let b = (color.z() * 255.999) as i32;
    writeln!(file, "{} {} {}", r, g, b)?;
    Ok(())
}

fn hit_sphere(center: &Point, radius: f32, ray: &Ray) -> bool {
    // Vector OC
    let oc = *ray.origin() - *center;
    let a = ray.direction().dot(ray.direction());
    let b = 2. * oc.dot(ray.direction());
    let c = oc.dot(&oc) - radius * radius;

    return (b * b - 4. * a * c) > 0.;
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point::new(0., 0., -1.), 0.5, ray) {
        return Color::new(1., 0., 0.);
    }
    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);

    let max_y = Color::new(0.5, 0.7, 1.); // Blue
    let min_y = Color::new(1., 1., 1.); // White

    // Lerp pixel color based on distance to camera
    return (1. - t) * min_y + t * max_y;
}

fn main() -> Result<()> {
    // Image settings
    const ASPECT_RATIO: f32 = 16. / 9.;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
    const MAX_COLOR: i32 = 255;

    // Camera settings
    const VIEWPORT_HEIGHT: f32 = 2.;
    const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f32 = 1.;

    let origin = Point::new(0., 0., 0.);
    let horizontal = Vec3::new(VIEWPORT_WIDTH, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT, 0.);
    let lower_left_corner: Point =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., FOCAL_LENGTH);

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
            let u = i as f32 / (IMAGE_WIDTH - 1) as f32;
            let v = j as f32 / (IMAGE_HEIGHT - 1) as f32;
            let horizontal_offset = u * horizontal;
            let vertical_offset = v * vertical;

            let ray = Ray::new(
                origin,
                lower_left_corner + horizontal_offset + vertical_offset - origin,
            );
            let pixel = ray_color(&ray);
            write_color(&file, &pixel)?;
        }
    }
    println!("\nDone!");
    Ok(())
}
