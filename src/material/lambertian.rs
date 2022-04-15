use crate::{
    object::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{ScatterResult, Scatterable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Scatterable for Lambertian {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let mut scatter_direction = record.normal() + Vec3::new_random_unit_vector();
        if scatter_direction.is_near_zero() {
            scatter_direction = record.normal();
        }
        Some(ScatterResult {
            ray: Ray::new(record.point(), scatter_direction),
            attenuation: self.albedo,
        })
    }
}
