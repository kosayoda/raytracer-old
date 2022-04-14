use crate::material::Material;
use crate::types::ray::Ray;
use crate::types::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct HitRecord {
    point: Point,
    normal: Vec3,
    t: f32,
    is_front_face: bool,
    material: &'static Material,
}

impl HitRecord {
    pub fn new(
        point: Point,
        normal: Vec3,
        t: f32,
        is_front_face: bool,
        material: &'static Material,
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

    pub fn material(self) -> &'static Material {
        self.material
    }

    pub fn is_front_face(self) -> bool {
        self.is_front_face
    }
}

pub trait Hittable {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Hittable for Vec<Box<dyn Hittable + Send + Sync>> {
    fn hit(&self, ray: Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_hit = None;
        for hittable in self {
            // If we hit something
            if let Some(h) = hittable.hit(ray, t_min, t_max) {
                // Compare with the current closest hit
                match closest_hit {
                    None => closest_hit = Some(h),
                    Some(c) => {
                        if h.t < c.t {
                            closest_hit = Some(h);
                        }
                    }
                }
            }
        }
        closest_hit
    }
}
