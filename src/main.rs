use std::fs::File;
use std::io::{BufWriter, Write};

use anyhow::Result;
use rand::Rng;

use raytracer::camera::Camera;
use raytracer::hittable::Hittable;
use raytracer::ray::Ray;
use raytracer::sphere::Sphere;
use raytracer::vec3::{Color, Point};

// Image settings
const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const MAX_COLOR: i32 = 255;
const SAMPLES_PER_PIXEL: i32 = 100;

// Camera settings
const VIEWPORT_HEIGHT: f32 = 2.;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.;

fn write_color(file: &mut BufWriter<File>, color: &Color, scale: f32) -> Result<()> {
    // Scale the colors
    let _r = color.x() * scale;
    let _g = color.y() * scale;
    let _b = color.z() * scale;

    // Clamp the colors to [0, 255]
    let r = (256. * _r.clamp(0., 0.999)) as i32;
    let g = (256. * _g.clamp(0., 0.999)) as i32;
    let b = (256. * _b.clamp(0., 0.999)) as i32;
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
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH);

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
    let f = File::create(name).expect("Failed to create file!");
    let mut file = BufWriter::new(f);

    // Write header
    writeln!(file, "P3")?;
    writeln!(file, "{} {}", IMAGE_WIDTH, IMAGE_HEIGHT)?;
    writeln!(file, "{}", MAX_COLOR)?;

    // Write data
    let mut rng = rand::thread_rng();

    let max_u = (IMAGE_WIDTH - 1) as f32;
    let max_v = (IMAGE_HEIGHT - 1) as f32;

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        let _j = j as f32;
        for i in 0..IMAGE_WIDTH {
            let _i = i as f32;
            let mut pixel = Color::new(0., 0., 0.);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (_i + rng.gen::<f32>()) / max_u;
                let v = (_j + rng.gen::<f32>()) / max_v;

                let ray = (&camera).get_ray(u, v);
                pixel = pixel + ray_color(ray, &world);
            }
            write_color(&mut file, &pixel, 1. / SAMPLES_PER_PIXEL as f32)?;
        }
    }
    eprintln!("\nDone!");
    Ok(())
}
