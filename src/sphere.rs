use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Self {
        Self { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Vector OC
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let b = oc.dot(ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discrim = b * b - a * c;
        if discrim > 0. {
            let t = (-b - discrim.sqrt()) / a;
            if t_min < t && t < t_max {
                let p = ray.at(t);
                return Some(HitRecord::new(p, (p - self.center) / self.radius, t));
            }
        }
        None
    }
}
