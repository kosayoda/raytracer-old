use crate::{
    object::HitRecord,
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::{ScatterResult, Scatterable};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = r_in.direction().unit_vector().reflect(record.normal());
        let scattered = Ray::new(
            record.point(),
            reflected + self.fuzz * Vec3::new_random_in_unit_sphere(),
        );

        if scattered.direction().dot(record.normal()) > 0. {
            Some(ScatterResult {
                ray: scattered,
                attenuation: self.albedo,
            })
        } else {
            None
        }
    }
}
