use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use anyhow::Result;
use clap::{arg, Command};

use raytracer::camera::Camera;
use raytracer::config::RaytracerConfig;
use raytracer::tracer::Tracer;

fn main() -> Result<()> {
    let matches = Command::new("raytracer")
        .arg(arg!(<scene> "The scene to render"))
        .arg(arg!(-s --save <save> "The path to save the render").required(false))
        .get_matches();

    let scene = matches.value_of("scene").map(PathBuf::from).unwrap();
    let reader = BufReader::new(File::open(scene)?);
    let config: RaytracerConfig = serde_json::from_reader(reader)?;

    let aspect_ratio: f32 = config.image_width as f32 / config.image_height as f32;
    let camera = Camera::new(
        config.look_from,
        config.look_to,
        config.viewport_fov,
        aspect_ratio,
        config.aperture,
        config.focal_length,
    );

    let tracer = Tracer::new(camera, config);

    let save_path = matches
        .value_of("save")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("image.png"));
    tracer.save(&save_path)?;

    Ok(())
}
