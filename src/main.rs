use std::fs::File;
use std::io::Write;

use anyhow::Result;

use raytracer::hittable::Hittable;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::{Color, Point, Vec3};

fn write_color(mut file: &File, color: &Color) -> Result<()> {
    let r = (color.x() * 255.999) as i32;
    let g = (color.y() * 255.999) as i32;
    let b = (color.z() * 255.999) as i32;
    writeln!(file, "{} {} {}", r, g, b)?;
    Ok(())
}

fn ray_color(ray: Ray, world: &dyn Hittable) -> Color {
    if let Some(record) = world.hit(ray, 0., f32::MAX) {
        return 0.5 * (record.normal() + Color::new(1., 1., 1.));
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);

    let max_y = Color::new(0.5, 0.7, 1.); // Blue
    let min_y = Color::new(1., 1., 1.); // White

    // Lerp pixel color based on distance to camera
    (1. - t) * min_y + t * max_y
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

    let spheres = vec![
        Sphere::new(Point::new(0., 0., -1.), 0.5),
        Sphere::new(Point::new(0., -100.5, -1.), 100.),
    ];
    let world: Vec<Box<dyn Hittable>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable>)
        .collect();

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
            let pixel = ray_color(ray, &world);
            write_color(&file, &pixel)?;
        }
    }
    println!("\nDone!");
    Ok(())
}
