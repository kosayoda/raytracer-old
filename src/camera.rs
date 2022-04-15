use crate::primitive::ray::Ray;
use crate::primitive::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(viewport_fov: f32, aspect_ratio: f32) -> Self {
        let theta = viewport_fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

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
