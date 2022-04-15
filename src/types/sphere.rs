use super::ray::Ray;
use super::vec3::Point;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::vec3::Vec3;

pub struct Sphere {
    center: Point,
    radius: f32,
    material: &'static Material,
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: &'static Material) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // Sphere equation:
        // x² + y² + z² = (x - Cx)² + (y - Cy)² + (z - Cz)²
        //                 = (P - C) · (P - C)
        // where P is a point and C is the center of the sphere
        //
        // If P is on the sphere, (P - C) · (P - C) = R²
        //
        // As P is a point along the ray P(t) where some arbitrary t,
        // and P(t) = A + tv where A is the origin and v is the ray direction
        // the LHS of the equation expands to at² + bt + c where
        // a = (v • v), b = (A - C) • 2v, c = (A - C) • (A - C) - r^2
        //
        // Let OC = A - C and h = OC · v
        // Given that b = 2(OC · v), the quadratic formula
        // t = (-b ± sqrt(b^2 - 4ac)) / 2a can be simplified to
        // t = (-h ± sqrt(h^2 - ac)) / a

        let oc = ray.origin() - self.center;
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
        let outward_normal = (point - self.center) / self.radius;
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
