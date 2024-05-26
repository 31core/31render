use rand::Rng;
use std::ops::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct Vector3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector3D {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    /**
     * Generate a random unit vector
     */
    pub fn new_random_unit() -> Self {
        let mut rng = rand::thread_rng();
        let rand_vec = Vector3D {
            x: rng.gen_range(-1.0..1.),
            y: rng.gen_range(-1.0..1.),
            z: rng.gen_range(-1.0..1.),
        };
        rand_vec.unit()
    }
    /**
     * Dot product
     */
    pub fn cdot(&self, rhs: &Self) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    /**
     * Calculate the module lenth
     */
    pub fn module(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }
    /**
     * Calculate the unit vector
     */
    pub fn unit(&self) -> Self {
        *self / self.module()
    }
}

impl From<(f64, f64, f64)> for Vector3D {
    fn from(vec: (f64, f64, f64)) -> Vector3D {
        Vector3D {
            x: vec.0,
            y: vec.1,
            z: vec.2,
        }
    }
}

impl From<Vector3D> for (f64, f64, f64) {
    fn from(vec: Vector3D) -> (f64, f64, f64) {
        (vec.x, vec.y, vec.z)
    }
}

impl Add for Vector3D {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}

impl Add<f64> for Vector3D {
    type Output = Self;
    fn add(mut self, rhs: f64) -> Self::Output {
        self.x += rhs;
        self.y += rhs;
        self.z += rhs;
        self
    }
}

impl AddAssign for Vector3D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vector3D {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}

impl Sub<f64> for Vector3D {
    type Output = Self;
    fn sub(mut self, rhs: f64) -> Self::Output {
        self.x -= rhs;
        self.y -= rhs;
        self.z -= rhs;
        self
    }
}

impl Mul for Vector3D {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Vector3D::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Mul<Vector3D> for f64 {
    type Output = Vector3D;
    fn mul(self, mut rhs: Vector3D) -> Self::Output {
        rhs.x *= self;
        rhs.y *= self;
        rhs.z *= self;
        rhs
    }
}

impl Div<f64> for Vector3D {
    type Output = Self;
    fn div(mut self, rhs: f64) -> Self {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self
    }
}

impl Neg for Vector3D {
    type Output = Self;
    fn neg(self) -> Self {
        -1. * self
    }
}
