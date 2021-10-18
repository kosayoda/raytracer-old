use crate::types::ray::Ray;
use crate::types::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(viewport_width: f32, viewport_height: f32, focal_length: f32) -> Self {
        let origin = Point::new(0., 0., 0.);
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let distance = Vec3::new(0., 0., focal_length);

        let lower_left = origin - horizontal / 2. - vertical / 2. - distance;
        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
        }
    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        let horizontal_offset = u * self.horizontal;
        let vertical_offset = v * self.vertical;
        Ray::new(
            self.origin,
            self.lower_left + horizontal_offset + vertical_offset - self.origin,
        )
    }
}
