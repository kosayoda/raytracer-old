use super::vec3::{Point, Vec3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vec3,
    time: f32,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3, time: f32) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    pub fn origin(self) -> Point {
        self.origin
    }

    pub fn direction(self) -> Vec3 {
        self.direction
    }

    /// Get the point along the vector at a certain param t
    pub fn at(self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn time(&self) -> f32 {
        self.time
    }
}
