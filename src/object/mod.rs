use serde::{Deserialize, Serialize};

use enum_dispatch::enum_dispatch;

use crate::material::Material;
use crate::primitive::ray::Ray;
use crate::primitive::vec3::{Point, Vec3};

mod moving_sphere;
mod sphere;
pub use moving_sphere::MovingSphere;
pub use sphere::Sphere;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    point: Point,
    normal: Vec3,
    t: f32,
    is_front_face: bool,
    material: Material,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vec3,
        t: f32,
        is_front_face: bool,
        material: Material,
    ) -> Self {
        Self {
            point,
            normal,
            t,
            is_front_face,
            material,
        }
    }

    pub fn point(self) -> Point {
        self.point
    }

    pub fn normal(self) -> Vec3 {
        self.normal
    }

    pub fn t(self) -> f32 {
        self.t
    }

    pub fn material(self) -> Material {
        self.material
    }

    pub fn is_front_face(self) -> bool {
        self.is_front_face
    }
}

#[enum_dispatch]
pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

#[enum_dispatch(Hittable)]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Object {
    Sphere,
}

impl Hittable for Vec<Object> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            // If we hit something
            if let Some(h) = hittable.hit(ray, t_min, closest_so_far) {
                closest_hit = Some(h);
                closest_so_far = h.t;
            }
        }
        closest_hit
    }
}

impl Hittable for [Object] {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        let mut closest_so_far = t_max;

        for hittable in self {
            // If we hit something
            if let Some(h) = hittable.hit(ray, t_min, closest_so_far) {
                closest_hit = Some(h);
                closest_so_far = h.t;
            }
        }
        closest_hit
    }
}
