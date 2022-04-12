use std::path::Path;

use anyhow::Result;

use raytracer::camera::Camera;
use raytracer::hittable::Hittable;
use raytracer::sphere::Sphere;
use raytracer::tracer::{Tracer, TracerConfig};
use raytracer::vec3::Point;

// Image settings
const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

const MAX_DEPTH: i32 = 50;

// Camera settings
const VIEWPORT_HEIGHT: f32 = 2.;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.;

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

    let config = TracerConfig::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    let tracer = Tracer::new(world, camera, config);

    tracer.save(Path::new("image.png"))?;
    eprintln!("\nDone!");
    Ok(())
}
