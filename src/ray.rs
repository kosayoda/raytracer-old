use crate::vec::{Point, Vec3};

pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get a reference to the ray's origin.
    pub fn origin(&self) -> &Point {
        &self.origin
    }

    /// Get a reference to the ray's direction.
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn at(self, t: f32) -> Point {
        self.origin + t * self.direction
    }
}
