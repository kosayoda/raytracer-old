use std::path::Path;

use anyhow::Result;
use once_cell::sync::Lazy;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

use raytracer::camera::Camera;
use raytracer::material::{Dielectric, Lambertian, Material, Metal};
use raytracer::object::{Object, Sphere};
use raytracer::tracer::{Tracer, TracerConfig};
use raytracer::vec3::{Color, Point, Vec3};

// Image settings
const ASPECT_RATIO: f32 = 3. / 2.;
const IMAGE_WIDTH: i32 = 1200;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f32 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 500;

const MAX_DEPTH: i32 = 50;

// Create materials
static MATERIAL_GROUND: Lazy<Material> = Lazy::new(|| {
    Material::from(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    })
});

fn random_scene() -> Vec<Object> {
    let mut world: Vec<Object> = vec![Object::from(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        *MATERIAL_GROUND,
    ))];

    let mut rng = SmallRng::from_entropy();

    for _a in -11..11 {
        for _b in -11..11 {
            let a = _a as f32;
            let b = _b as f32;
            let choose_mat = rng.gen::<f32>();
            let center = Point::new(a + 0.9 * rng.gen::<f32>(), 0.2, b + 0.9 * rng.gen::<f32>());

            if (center - Point::new(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = Color::new_random() * Color::new_random();
                    world.push(Object::from(Sphere::new(
                        center,
                        0.2,
                        Material::from(Lambertian { albedo }),
                    )))
                } else if choose_mat < 0.95 {
                    world.push(Object::from(Sphere::new(
                        center,
                        0.2,
                        Material::from(Dielectric {
                            refractive_index: 1.5,
                        }),
                    )))
                } else {
                }
            }
        }
    }
    world.push(Object::from(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        Material::from(Dielectric {
            refractive_index: 1.5,
        }),
    )));
    world.push(Object::from(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        Material::from(Lambertian {
            albedo: Color::new(0.4, 0.2, 0.1),
        }),
    )));
    world.push(Object::from(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        Material::from(Metal {
            albedo: Color::new(0.7, 0.6, 0.5),
            fuzz: 0.0,
        }),
    )));
    world
}

fn main() -> Result<()> {
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_at = Point::new(0., 0., 0.);
    let vup = Vec3::new(0., 1., 0.);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        look_from,
        look_at,
        vup,
        20.,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
    );

    let world = random_scene();
    let config = TracerConfig::new(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL, MAX_DEPTH);
    let tracer = Tracer::new(world, camera, config);

    tracer.save(Path::new("image.png"))?;
    eprintln!("\nDone!");
    Ok(())
}
