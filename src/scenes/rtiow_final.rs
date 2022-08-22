use once_cell::sync::Lazy;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

use crate::config::RaytracerConfig;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::object::{Object, Sphere};
use crate::vec3::{Color, Point};

// Create materials
static MATERIAL_GROUND: Lazy<Material> = Lazy::new(|| {
    Material::from(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    })
});

pub fn scene() -> RaytracerConfig {
    // -- World --
    let mut world: Vec<Object> = Vec::new();
    world.push(Object::from(Sphere::new(
        Point::new(0., -1000., 0.),
        1000.,
        *MATERIAL_GROUND,
    )));

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

    // -- Configuration
    let image_width = 1200;
    let image_height = 800;
    let samples_per_pixel = 500;
    let max_depth = 50;

    let viewport_fov = 90.0;
    let aperture = 0.1;
    let focal_length = Some(10.0);
    let look_from = Point::new(13.0, 2.0, 3.0);
    let look_to = Point::new(0.0, 0.0, 0.0);

    RaytracerConfig {
        image_width,
        image_height,
        samples_per_pixel,
        max_depth,
        viewport_fov,
        aperture,
        focal_length,
        look_from,
        look_to,
        world,
    }
}
