use crate::primitive::ray::Ray;
use crate::primitive::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f32,
}

impl Camera {
    pub fn new(
        look_from: Point,
        look_at: Point,
        vup: Vec3,
        viewport_fov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
    ) -> Self {
        let h = (viewport_fov.to_radians() / 2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).unit_vector();
        let u = vup.cross(w).unit_vector();
        let v = w.cross(u);

        let origin = look_from;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        let lower_left = origin - horizontal / 2. - vertical / 2. - focus_dist * w;
        let lens_radius = aperture / 2.;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vec3::new_random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        let horizontal_offset = u * self.horizontal;
        let vertical_offset = v * self.vertical;
        Ray::new(
            self.origin + offset,
            self.lower_left + horizontal_offset + vertical_offset - self.origin - offset,
        )
    }
}
