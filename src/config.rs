use serde::{Deserialize, Serialize};

use crate::object::Object;
use crate::vec3::Point;

#[derive(Serialize, Deserialize, Debug)]
pub struct RaytracerConfig {
    #[serde(default = "default_image_width")]
    pub image_width: i32,
    #[serde(default = "default_image_height")]
    pub image_height: i32,
    #[serde(default = "default_samples_per_pixel")]
    pub samples_per_pixel: i32,
    #[serde(default = "default_max_depth")]
    pub max_depth: i32,
    #[serde(default = "default_viewport_fov")]
    pub viewport_fov: f32,
    #[serde(default = "default_aperture")]
    pub aperture: f32,
    pub focal_length: Option<f32>,
    pub look_from: Point,
    pub look_to: Point,
    pub world: Vec<Object>,
}

fn default_image_width() -> i32 {
    400
}

fn default_image_height() -> i32 {
    225
}

fn default_samples_per_pixel() -> i32 {
    300
}

fn default_max_depth() -> i32 {
    50
}

fn default_viewport_fov() -> f32 {
    90.0
}

fn default_aperture() -> f32 {
    0.0
}
