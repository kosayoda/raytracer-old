use std::ops::{Add, Div, Mul, Sub};

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

// Vector arithmetic
impl Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

// Scalar arithmetic
impl Mul<f32> for Vec3 {
    type Output = Self;
    fn mul(self, scalar: f32) -> Self {
        Self {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vector: Vec3) -> Vec3 {
        vector * self
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;
    fn div(self, scalar: f32) -> Self {
        Self {
            x: self.x * 1. / scalar,
            y: self.y * 1. / scalar,
            z: self.z * 1. / scalar,
        }
    }
}

impl Vec3 {
    /// Create a new Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Dot product of two vectors
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product of two vectors
    pub fn cross(self, other: Vec3) -> Vec3 {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.y * other.x - self.x * other.y,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Unit vector of the vector
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    /// Length vector of the vector
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Get a reference to the vec3's z.
    pub fn z(&self) -> &f32 {
        &self.z
    }

    /// Get a reference to the vec3's x.
    pub fn x(&self) -> &f32 {
        &self.x
    }

    /// Get a reference to the vec3's y.
    pub fn y(&self) -> &f32 {
        &self.y
    }
}
