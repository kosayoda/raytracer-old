use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;

use raytracer::camera::Camera;
use raytracer::hittable::Hittable;
use raytracer::material::{Dielectric, Lambertian, Material, Metal};
use raytracer::sphere::Sphere;
use raytracer::tracer::{Tracer, TracerConfig};
use raytracer::vec3::{Color, Point};

// Image settings
const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 50;

const MAX_DEPTH: i32 = 100;

// Camera settings
const VIEWPORT_HEIGHT: f32 = 2.;
const VIEWPORT_WIDTH: f32 = ASPECT_RATIO * VIEWPORT_HEIGHT;
const FOCAL_LENGTH: f32 = 1.;

fn main() -> Result<()> {
    let camera = Camera::new(VIEWPORT_WIDTH, VIEWPORT_HEIGHT, FOCAL_LENGTH);

    // Create materials
    static MATERIAL_GROUND: Lazy<Material> = Lazy::new(|| {
        Material::from(Lambertian {
            albedo: Color::new(0.8, 0.8, 0.0),
        })
    });
    static MATERIAL_CENTER: Lazy<Material> = Lazy::new(|| {
        Material::from(Lambertian {
            albedo: Color::new(0.1, 0.2, 0.5),
        })
    });
    static MATERIAL_LEFT: Lazy<Material> = Lazy::new(|| {
        Material::from(Dielectric {
            refractive_index: 1.5,
        })
    });
    static MATERIAL_RIGHT: Lazy<Material> = Lazy::new(|| {
        Material::from(Metal {
            albedo: Color::new(0.8, 0.6, 0.2),
            fuzz: 0.0,
        })
    });
    let spheres = vec![
        Sphere::new(Point::new(0.0, -100.5, -1.0), 100., &MATERIAL_GROUND),
        Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5, &MATERIAL_CENTER),
        Sphere::new(Point::new(-1.0, 0.0, -1.0), 0.5, &MATERIAL_LEFT),
        Sphere::new(Point::new(1.0, 0.0, -1.0), 0.5, &MATERIAL_RIGHT),
    ];
    let world: Vec<Box<dyn Hittable + Sync + Send>> = spheres
        .into_iter()
        .map(|s| Box::new(s) as Box<dyn Hittable + Sync + Send>)
        .collect();

    let config = TracerConfig::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    let tracer = Tracer::new(world, camera, config);

    tracer.save(Path::new("image.png"))?;
    eprintln!("\nDone!");
    Ok(())
}
