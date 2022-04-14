use enum_dispatch::enum_dispatch;
use rand::prelude::SmallRng;
use rand::{Rng, SeedableRng};

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
    Dielectric,
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

fn refract(vector: &Vec3, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = (-*vector).dot(*normal).min(1.);
    let r_out_perpendicular = etai_over_etat * (*vector + *normal * cos_theta);
    let r_out_parallel = *normal * -(1. - r_out_perpendicular.length_squared()).abs().sqrt();
    r_out_perpendicular + r_out_parallel
}

fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
    // Shlick's approximation
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * f32::powi(1. - cosine, 5)
}

#[derive(Debug, PartialEq)]
pub struct Dielectric {
    pub refractive_index: f32,
}

impl Scatterable for Dielectric {
    fn scatter(&self, r_in: &Ray, record: &HitRecord) -> Option<ScatterResult> {
        let refraction_ratio = if record.is_front_face() {
            1. / self.refractive_index
        } else {
            self.refractive_index
        };

        let unit_direction = r_in.direction().unit_vector();

        let cos_theta = (-unit_direction).dot(record.normal()).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let mut rng = SmallRng::from_entropy();

        // Cannot refract
        let direction = if (refraction_ratio * sin_theta) > 1.0
            || reflectance(cos_theta, refraction_ratio) > rng.gen::<f32>()
        {
            reflect(&unit_direction, &record.normal())
        } else {
            refract(&unit_direction, &record.normal(), refraction_ratio)
        };

        Some(ScatterResult {
            ray: Ray::new(record.point(), direction),
            attenuation: Color::new(1., 1., 1.),
        })
    }
}
