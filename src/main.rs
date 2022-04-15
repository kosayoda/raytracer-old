use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;

use raytracer::camera::Camera;
use raytracer::material::{Dielectric, Lambertian, Material, Metal};
use raytracer::object::{Object, Sphere};
use raytracer::tracer::{Tracer, TracerConfig};
use raytracer::vec3::{Color, Point, Vec3};

// Image settings
const ASPECT_RATIO: f32 = 16. / 9.;
const IMAGE_WIDTH: i32 = 400;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;

const MAX_DEPTH: i32 = 100;

fn main() -> Result<()> {
    let look_from = Point::new(3., 3., 2.);
    let look_at = Point::new(0., 0., -1.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = (look_from - look_at).length();
    let aperture = 2.0;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

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
    let world = vec![
        Object::from(Sphere::new(
            Point::new(0.0, -100.5, -1.0),
            100.,
            &MATERIAL_GROUND,
        )),
        Object::from(Sphere::new(
            Point::new(0.0, 0.0, -1.0),
            0.5,
            &MATERIAL_CENTER,
        )),
        Object::from(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            0.5,
            &MATERIAL_LEFT,
        )),
        Object::from(Sphere::new(
            Point::new(-1.0, 0.0, -1.0),
            -0.45,
            &MATERIAL_LEFT,
        )),
        Object::from(Sphere::new(
            Point::new(1.0, 0.0, -1.0),
            0.5,
            &MATERIAL_RIGHT,
        )),
    ];

    let config = TracerConfig::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    let tracer = Tracer::new(world, camera, config);

    tracer.save(Path::new("image.png"))?;
    eprintln!("\nDone!");
    Ok(())
}
