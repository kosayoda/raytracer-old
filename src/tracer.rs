use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::str::FromStr;

use anyhow::{anyhow, Result};
use flate2::write::ZlibEncoder;
use flate2::Compression;
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use rayon::prelude::*;

use crate::camera::Camera;
use crate::hittable::Hittable;
use crate::material::Scatterable;
use crate::png::Chunk;
use crate::png::ChunkType;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

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
    world: Vec<Box<dyn Hittable + Send + Sync>>,
    camera: Camera,
    config: TracerConfig,
    current_x: i32,
    current_y_forward: i32,
    current_y_backward: i32,
}

impl Tracer {
    pub fn new(
        world: Vec<Box<dyn Hittable + Send + Sync>>,
        camera: Camera,
        config: TracerConfig,
    ) -> Self {
        Self {
            world,
            camera,
            current_x: 0,
            current_y_forward: config.height - 1,
            current_y_backward: 0,
            config,
        }
    }

    pub fn save(self, filepath: &Path) -> Result<()> {
        let file = File::create(filepath)?;

        if let Some(ext) = filepath.extension().and_then(|s| s.to_str()) {
            match ext {
                "ppm" => self.save_as_ppm(file)?,
                "bmp" => self.save_as_bmp(file)?,
                "png" => self.save_as_png(file)?,
                _ => return Err(anyhow!("Unsupported filetype!")),
            }
        } else {
            println!("No filetype given, defaulting to ppm...");
            self.save_as_ppm(file)?;
        }

        Ok(())
    }

    fn save_as_ppm<W: Write>(self, writable: W) -> Result<()> {
        let mut file = BufWriter::new(writable);

        // Write header
        writeln!(file, "P3")?;
        writeln!(file, "{} {}", self.config.width, self.config.height)?;
        writeln!(file, "{}", 255)?; // Maximum color

        // Write pixels
        let samples_per_pixel: i32 = self.config.samples_per_pixel;
        for mut pixel in self {
            pixel.correct_color(1. / samples_per_pixel as f32);
            writeln!(file, "{} {} {}", pixel.r(), pixel.g(), pixel.b())?;
        }

        Ok(())
    }

    fn save_as_bmp<W: Write>(self, writable: W) -> Result<()> {
        let mut file = BufWriter::new(writable);

        let mut bitmap_file_header: [u8; 14] = [
            0x42, 0x4D, // BM marker
            0xFF, 0xFF, 0xFF, 0xFF, // BMP file size in bytes
            0x00, 0x00, 0x00, 0x00, // Reserved
            0x36, 0x00, 0x00, 0x00, // Byte offset of the pixel array
        ];
        let mut dib_header: [u8; 40] = [
            0x28, 0x00, 0x00, 0x00, // DIB header size in bytes
            0xFF, 0xFF, 0xFF, 0xFF, // Bitmap pixel width
            0xFF, 0xFF, 0xFF, 0xFF, // Bitmap pixel height
            0x01, 0x00, // Number of color planes
            0xFF, 0xFF, // Number of bits per pixel
            0x00, 0x00, 0x00, 0x00, // BI_RGB, no pixel array compression
            0xFF, 0xFF, 0xFF, 0xFF, // Size of raw bitmap data including padding
            0x00, 0x00, 0x00, 0x00, // Print resolution (vertical)
            0x00, 0x00, 0x00, 0x00, // Print resolution (horizontal)
            0x00, 0x00, 0x00, 0x00, // Number of colors in the palette
            0x00, 0x00, 0x00, 0x00, // Important colors (0 means all important)
        ];

        let bits_per_pixel: i32 = 24;
        let row_size = (((bits_per_pixel * self.config.width) as f32 / 32.).ceil() * 4.) as usize;
        let data_size = row_size * self.config.height as usize;

        dib_header[4..=7].copy_from_slice(&(self.config.width as u32).to_le_bytes());
        dib_header[8..=11].copy_from_slice(&(self.config.height as u32).to_le_bytes());
        dib_header[14..=15].copy_from_slice(&(bits_per_pixel as u16).to_le_bytes());
        dib_header[20..=23].copy_from_slice(&(data_size as u32).to_le_bytes());

        let total_size = data_size + dib_header.len() + bitmap_file_header.len();
        bitmap_file_header[2..=5].copy_from_slice(&(total_size as u32).to_le_bytes());

        file.write_all(&bitmap_file_header)?;
        file.write_all(&dib_header)?;

        let padding_required = (4 - self.config.width % 4) % 4;
        let samples_per_pixel: i32 = self.config.samples_per_pixel;
        let width = self.config.width as usize;
        for (idx, mut pixel) in self.rev().enumerate() {
            pixel.correct_color(1. / samples_per_pixel as f32);
            file.write_all(&[pixel.b(), pixel.g(), pixel.r()])?;

            // Write padding every row
            if idx % width == 0 {
                for _ in 0..padding_required {
                    file.write_all(&[0_u8])?;
                }
            }
        }

        Ok(())
    }

    // PNG Format taken from: http://www.libpng.org/pub/png/spec/1.2/PNG-Contents.html
    // PNG Chunk/ChunkType scaffold + tests taken from https://picklenerd.github.io/pngme_book/
    fn save_as_png<W: Write>(self, writable: W) -> Result<()> {
        let mut file = BufWriter::new(writable);

        // Write PNG Header
        file.write_all(&[137, 80, 78, 71, 13, 10, 26, 10])?;

        // Write IHDR chunk
        let mut ihdr_data: [u8; 13] = [
            0xFF, 0xFF, 0xFF, 0xFF, // Pixel width
            0xFF, 0xFF, 0xFF, 0xFF, // Pixel height
            0x8,  // Bit depth (number of bits per sample, NOT per pixel)
            0x2,  // Color type
            0x0,  // Compression method
            0x0,  // Filter method
            0x0,  // Interlace method
        ];
        ihdr_data[0..=3].copy_from_slice(&(self.config.width as u32).to_be_bytes());
        ihdr_data[4..=7].copy_from_slice(&(self.config.height as u32).to_be_bytes());

        file.write_all(
            &Chunk::new(ChunkType::from_str("IHDR").unwrap(), ihdr_data.to_vec()).as_bytes(),
        )?;

        let mut e = ZlibEncoder::new(Vec::new(), Compression::default());

        let samples_per_pixel: i32 = self.config.samples_per_pixel;
        let data: Vec<u8> = (0..self.config.height)
            .into_par_iter()
            .rev()
            .flat_map(|y| (0..self.config.width).into_par_iter().map(move |x| (x, y)))
            .map_init(SmallRng::from_entropy, |rng, (x, y)| {
                let _i = x as f32;
                let _j = y as f32;
                let mut pixel = Color::new(0., 0., 0.);

                for _ in 0..samples_per_pixel {
                    let u = (_i + rng.gen::<f32>()) / self.config.max_u;
                    let v = (_j + rng.gen::<f32>()) / self.config.max_v;
                    let ray = (&self.camera).get_ray(u, v);
                    pixel = pixel + ray_color(ray, &self.world, self.config.max_depth);
                }

                // Write filter-type byte every row
                pixel.correct_color(1. / samples_per_pixel as f32);
                [pixel.r(), pixel.g(), pixel.b()]
            })
            .flatten()
            .collect();

        let width = self.config.width as usize;
        for (index, pixel) in data.chunks(3).enumerate() {
            if index % width == 0 {
                e.write_all(&[0])?;
            }
            e.write_all(pixel)?;
        }

        let compressed = e.finish()?;
        file.write_all(&Chunk::new(ChunkType::from_str("IDAT").unwrap(), compressed).as_bytes())?;

        // Write IEND chunk
        file.write_all(&Chunk::new(ChunkType::from_str("IEND").unwrap(), [].to_vec()).as_bytes())?;

        Ok(())
    }
}

impl Iterator for Tracer {
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        // No more lines
        if self.current_y_forward == 0 && self.current_x == self.config.width {
            return None;
        }
        // Move to next line
        if self.current_x == self.config.width {
            eprint!("\rScanlines remaining: {}", self.current_y_forward);
            self.current_y_forward -= 1;
            self.current_x = 0;
        }

        let _j = self.current_y_forward as f32;
        let _i = self.current_x as f32;
        let mut pixel = Color::new(0., 0., 0.);

        let mut rng = SmallRng::from_entropy();
        for _ in 0..self.config.samples_per_pixel {
            let u = (_i + rng.gen::<f32>()) / self.config.max_u;
            let v = (_j + rng.gen::<f32>()) / self.config.max_v;
            let ray = (&self.camera).get_ray(u, v);
            pixel = pixel + ray_color(ray, &self.world, self.config.max_depth);
        }

        self.current_x += 1;

        Some(pixel)
    }
}

impl DoubleEndedIterator for Tracer {
    fn next_back(&mut self) -> Option<Self::Item> {
        // No more lines
        if self.current_y_backward == self.config.height + 1 && self.current_x == self.config.width
        {
            return None;
        }

        // Move to next line
        if self.current_x == self.config.width {
            eprint!(
                "\rScanlines remaining: {}",
                self.config.height - self.current_y_backward
            );
            self.current_y_backward += 1;
            self.current_x = 0;
        }

        let mut rng = SmallRng::from_entropy();
        let _j = self.current_y_backward as f32;
        let _i = self.current_x as f32;
        let mut pixel = Color::new(0., 0., 0.);

        for _ in 0..self.config.samples_per_pixel {
            let u = (_i + rng.gen::<f32>()) / self.config.max_u;
            let v = (_j + rng.gen::<f32>()) / self.config.max_v;
            let ray = (&self.camera).get_ray(u, v);
            pixel = pixel + ray_color(ray, &self.world, self.config.max_depth);
        }

        self.current_x += 1;

        Some(pixel)
    }
}

fn ray_color(ray: Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0., 0., 0.);
    }

    if let Some(record) = world.hit(ray, 0.001, f32::MAX) {
        if let Some(res) = record.material().scatter(&ray, &record) {
            return res.attenuation * ray_color(res.ray, world, depth - 1);
        }
        return Color::new(0., 0., 0.);
    }

    let unit_direction = ray.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.);

    let min_y = Color::new(1., 1., 1.); // White
    let max_y = Color::new(0.5, 0.7, 1.); // Blue

    // Lerp pixel color based on distance to camera
    (1. - t) * min_y + t * max_y
}
