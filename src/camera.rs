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
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        viewport_fov: f32,
        aspect_ratio: f32,
    ) -> Self {
        let h = (viewport_fov.to_radians() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;

        let lower_left = origin - horizontal / 2. - vertical / 2. - w;
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
