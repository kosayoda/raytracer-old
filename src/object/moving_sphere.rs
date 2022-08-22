use std::ops::RangeInclusive;

use serde::{Deserialize, Serialize};

use crate::material::Material;
use crate::object::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MovingSphere {
    orig: Point,
    dest: Point,
    radius: f32,
    material: Material,
    time: RangeInclusive<f32>,
}

impl MovingSphere {
    pub fn new(
        orig: Point,
        dest: Point,
        radius: f32,
        material: Material,
        time: RangeInclusive<f32>,
    ) -> Self {
        Self {
            orig,
            dest,
            radius,
            material,
            time,
        }
    }

    pub fn center(&self, time: f32) -> Point {
        self.orig
            + ((time - self.time.start()) / (self.time.end() - self.time.start()))
                * (self.dest - self.orig)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
        // a = v • v = ∥v∥²
        let a = ray.direction().length_squared();
        // h = OC • v
        let h = oc.dot(ray.direction());
        // c = OC • OC - r² = ∥OC∥² - r²
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0. {
            return None;
        }

        // Real roots, ie. intersection of sphere
        let sqrt_d = discriminant.sqrt();
        // Obtain a root using quadratic formula
        let mut t = (-h - sqrt_d) / a;
        if t < t_min || t_max < t {
            t = (-h + sqrt_d) / a;
            if t < t_min || t_max < t {
                return None;
            }
        }

        let point = ray.at(t);
        let outward_normal = (point - self.center(ray.time())) / self.radius;
        let front_face = is_front_face(&ray, &outward_normal);
        let outward_normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        Some(HitRecord::new(
            point,
            outward_normal,
            t,
            front_face,
            self.material,
        ))
    }
}

fn is_front_face(ray: &Ray, outward_normal: &Vec3) -> bool {
    ray.direction().dot(*outward_normal) < 0.
}
