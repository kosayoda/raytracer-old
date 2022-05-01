use std::path::Path;

use anyhow::Result;

use raytracer::camera::Camera;
use raytracer::config::RaytracerConfig;
use raytracer::tracer::Tracer;

fn main() -> Result<()> {
    let json = r#"
        {
            "look_from": {"x": -2, "y": 2, "z": 1},
            "look_to": {"x": 0, "y": 0, "z": -1},
            "focal_length": 20,
            "aperture": 0,
            "world": [
                {
                  "Sphere": {
                    "center": {
                      "x": 0,
                      "y": -100.5,
                      "z": -1
                    },
                    "radius": 100,
                    "material": {
                      "Lambertian": {
                        "albedo": {
                          "x": 0.8,
                          "y": 0.8,
                          "z": 0
                        }
                      }
                    }
                  }
                },
                {
                  "Sphere": {
                    "center": {
                      "x": 0,
                      "y": 0,
                      "z": -1
                    },
                    "radius": 0.5,
                    "material": {
                      "Lambertian": {
                        "albedo": {
                          "x": 0.1,
                          "y": 0.2,
                          "z": 0.5
                        }
                      }
                    }
                  }
                },
                {
                  "Sphere": {
                    "center": {
                      "x": -1,
                      "y": 0,
                      "z": -1
                    },
                    "radius": 0.5,
                    "material": {
                      "Dielectric": {
                        "refractive_index": 0.5
                      }
                    }
                  }
                },
                {
                  "Sphere": {
                    "center": {
                      "x": -1,
                      "y": 0,
                      "z": -1
                    },
                    "radius": -0.45,
                    "material": {
                      "Dielectric": {
                        "refractive_index": 0.5
                      }
                    }
                  }
                },
                {
                  "Sphere": {
                    "center": {
                      "x": 1,
                      "y": 0,
                      "z": -1
                    },
                    "radius": 0.5,
                    "material": {
                      "Metal": {
                        "albedo": {
                          "x": 0.8,
                          "y": 0.6,
                          "z": 0.2
                        },
                        "fuzz": 0
                      }
                    }
                  }
                }
            ]
        }
    "#;
    let config: RaytracerConfig = serde_json::from_str(json)?;
    for object in &config.world {
        println!("{:?}", object);
    }

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

    tracer.save(Path::new("image.png"))?;
    eprintln!("\nDone!");
    Ok(())
}
