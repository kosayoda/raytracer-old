use enum_dispatch::enum_dispatch;

use crate::object::HitRecord;
use crate::ray::Ray;
use crate::vec3::Color;

mod dielectric;
mod lambertian;
mod metal;
pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

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
