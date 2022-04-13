use enum_dispatch::enum_dispatch;

use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Color, Vec3};

pub struct ScatterResult {
    pub attenuation: Color,
    pub ray: Ray,
}

#[enum_dispatch]
pub trait Scatterable {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterResult>;
}

#[enum_dispatch(Scatterable)]
#[derive(Debug, PartialEq)]
pub enum Material {
    Lambertian,
    Metal,
}

#[derive(Debug, PartialEq)]
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

fn reflect(vector: &Vec3, normal: &Vec3) -> Vec3 {
    *vector - (*normal * 2. * vector.dot(*normal))
}

#[derive(Debug, PartialEq)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f32,
}

impl Scatterable for Metal {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let reflected = reflect(&r_in.direction().unit_vector(), &record.normal());
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
