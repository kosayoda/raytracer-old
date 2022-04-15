use std::ops::{Add, Div, Mul, Neg, Sub};

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub type Point = Vec3;
pub type Color = Vec3;

#[derive(Debug, Default, Copy, Clone, PartialEq)]
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

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        self * -1.
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
            x: self.x * (1. / scalar),
            y: self.y * (1. / scalar),
            z: self.z * (1. / scalar),
        }
    }
}

impl Vec3 {
    /// Create a new Vec3
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Create a new Vec3 with random coordinates
    pub fn new_random(_min: f32, _max: f32) -> Self {
        let mut rng = SmallRng::from_entropy();
        Self {
            x: rng.gen_range(_min.._max),
            y: rng.gen_range(_min.._max),
            z: rng.gen_range(_min.._max),
        }
    }

    /// Create a new Vec3 with random coordinates
    pub fn new_random_in_unit_sphere() -> Self {
        let mut p;
        loop {
            p = Vec3::new_random(-1., 1.);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    /// Create a new Vec3 with random coordinates
    pub fn new_random_unit_vector() -> Self {
        Vec3::new_random_in_unit_sphere().unit_vector()
    }

    /// Create a new Vec3 with random coordinates
    pub fn new_random_in_hemisphere(normal: &Vec3) -> Self {
        let in_unit_sphere = Vec3::new_random_in_unit_sphere();
        if in_unit_sphere.dot(*normal) > 0. {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    /// Dot product of two vectors
    pub fn dot(self, other: Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Cross product of two vectors
    #[must_use]
    pub fn cross(self, other: Vec3) -> Vec3 {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.y * other.x - self.x * other.y,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Unit vector of the vector
    #[must_use]
    pub fn unit_vector(self) -> Self {
        self / self.length()
    }

    /// Length of the vector
    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Length of the vector squared
    pub fn length_squared(self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Whether the vector is close to zero in all dimensions
    pub fn is_near_zero(self) -> bool {
        self.x.abs() < 1e-8_f32 && self.y.abs() < 1e-8_f32 && self.z.abs() < 1e-8_f32
    }

    /// Get the vec3's x.
    pub fn x(self) -> f32 {
        self.x
    }

    /// Get the vec3's y.
    pub fn y(self) -> f32 {
        self.y
    }

    /// Get the vec3's z.
    pub fn z(self) -> f32 {
        self.z
    }
}

impl Color {
    pub fn correct_color(&mut self, scale: f32) {
        // Scale the colors
        let _r = self.x() * scale;
        let _g = self.y() * scale;
        let _b = self.z() * scale;

        // Clamp the colors to [0, 255]
        // Correct gamma for gamma 2.0
        self.x = 256. * _r.sqrt().clamp(0., 0.999);
        self.y = 256. * _g.sqrt().clamp(0., 0.999);
        self.z = 256. * _b.sqrt().clamp(0., 0.999);
    }

    pub fn r(self) -> u8 {
        self.x as u8
    }

    pub fn g(self) -> u8 {
        self.y as u8
    }

    pub fn b(self) -> u8 {
        self.z as u8
    }
}
