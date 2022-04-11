use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;

use anyhow::{anyhow, Result};
use rand::prelude::ThreadRng;
use rand::Rng;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

pub struct TracerConfig {
    width: i32,
    height: i32,
    max_u: f32,
    max_v: f32,
    samples_per_pixel: i32,
    max_depth: i32,
}

impl TracerConfig {
    pub fn new(width: i32, height: i32, samples_per_pixel: i32, max_depth: i32) -> Self {
        Self {
            width,
            height,
            samples_per_pixel,
            max_depth,
            max_u: (width - 1) as f32,
            max_v: (height - 1) as f32,
        }
    }
}

pub struct Tracer {
    world: Vec<Box<dyn Hittable>>,
    camera: Camera,
    rng: ThreadRng,
    config: TracerConfig,
    current_x: i32,
    current_y: i32,
}

impl Tracer {
    pub fn new(world: Vec<Box<dyn Hittable>>, camera: Camera, config: TracerConfig) -> Self {
        Self {
            world,
            camera,
            rng: rand::thread_rng(),
            current_x: 0,
            current_y: config.height - 1,
            config,
        }
    }

    pub fn save(self, filepath: &Path) -> Result<()> {
        let file = File::create(filepath)?;

        if let Some(ext) = filepath.extension().and_then(|s| s.to_str()) {
            match ext {
                "ppm" => self.save_as_ppm(file)?,
                _ => return Err(anyhow!("Unsupported filetype!")),
            }
        } else {
            println!("No filetype given, defaulting to ppm...");
            self.save_as_ppm(file)?;
        }

        Ok(())
    }

    fn save_as_ppm<W: Write>(self, writable: W) -> Result<()> {
        let mut writer = BufWriter::new(writable);

        // Write header
        writeln!(writer, "P3")?;
        writeln!(writer, "{} {}", self.config.width, self.config.height)?;
        writeln!(writer, "{}", 255)?; // Maximum color

        // Write pixels
        let samples_per_pixel: i32 = self.config.samples_per_pixel;
        for mut pixel in self {
            pixel.correct_color(1. / samples_per_pixel as f32);
            writeln!(writer, "{} {} {}", pixel.r(), pixel.g(), pixel.b())?;
        }

        Ok(())
    }
}

impl Iterator for Tracer {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        // No more lines
        if self.current_y == 0 && self.current_x == self.config.width {
            return None;
        }
        // Move to next line
        if self.current_x == self.config.width {
            eprint!("\rScanlines remaining: {}", self.current_y);
            self.current_y -= 1;
            self.current_x = 0;
        }

        let _j = self.current_y as f32;
        let _i = self.current_x as f32;
        let mut pixel = Color::new(0., 0., 0.);

        for _ in 0..self.config.samples_per_pixel {
            let u = (_i + self.rng.gen::<f32>()) / self.config.max_u;
            let v = (_j + self.rng.gen::<f32>()) / self.config.max_v;
            let ray = (&self.camera).get_ray(u, v);
            pixel = pixel + ray_color(ray, &self.world, &mut self.rng, self.config.max_depth);
        }

        self.current_x += 1;

        Some(pixel)
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable, rng: &mut ThreadRng, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }
    if let Some(record) = world.hit(ray, 0., f32::MAX) {
        let target = record.point() + record.normal() + Point::new_random_in_unit_sphere(rng);
        return 0.5
            * ray_color(
                Ray::new(record.point(), target - record.point()),
                world,
                rng,
                depth - 1,
            );
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);

    let max_y = Color::new(0.5, 0.7, 1.); // Blue
    let min_y = Color::new(1., 1., 1.); // White

    // Lerp pixel color based on distance to camera
    (1. - t) * min_y + t * max_y
}
