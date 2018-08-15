#![allow(dead_code)]

use std::fmt;
use std::ops;
use rand::{Rng, distributions::Uniform};

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    e: [f32; 3]
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { e: [x, y, z] }
    }

    pub fn one() -> Vector3 {
        Vector3::new(1.0f32, 1.0f32, 1.0f32)
    }

    pub fn zero() -> Vector3 {
        Vector3::new(0.0f32, 0.0f32, 0.0f32)
    }

    pub fn random<T: Rng>(rng: &mut T) -> Vector3 {
        let uniform = Uniform::new(0.0f32, 1.0f32);
        let mut result = None;
        let one = Vector3::one();
        while result.is_none() {
            let r = Vector3::new(rng.sample(uniform), rng.sample(uniform), rng.sample(uniform));
            let p = 2.0 * r - one;
            if p.length_squared() < 1.0 {
                result = Some(p)
            }
        }

        result.unwrap()
    }

    pub fn x(&self) -> f32 { return self.e[0]; }
    pub fn y(&self) -> f32 { return self.e[1]; }
    pub fn z(&self) -> f32 { return self.e[2]; }
    pub fn r(&self) -> f32 { return self.e[0]; }
    pub fn g(&self) -> f32 { return self.e[1]; }
    pub fn b(&self) -> f32 { return self.e[2]; }

    pub fn length_squared(self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn as_unit(self) -> Vector3 {
        let k = 1.0 / self.length();
        Vector3::new(
            self.e[0] * k,
            self.e[1] * k,
            self.e[2] * k,
        )
    }

    pub fn dot(self, rhs: Vector3) -> f32 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[0] * rhs.e[2] - self.e[2] * rhs.e[0],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
        )
    }

    pub fn lerp(t: f32, lhs: Vector3, rhs: Vector3) -> Vector3 {
        (1.0 - t) * lhs + t * rhs
    }

    pub fn to_rgb24(self) -> u32 {
        let r = (self.r() * 255.99f32).trunc() as u32;
        let g = (self.g() * 255.99f32).trunc() as u32;
        let b = (self.b() * 255.99f32).trunc() as u32;
        (r << 16) | (g << 8) | b
    }
}

impl ops::Add for Vector3 {
    type Output = Vector3;

    fn add(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::AddAssign for Vector3 {
    fn add_assign(&mut self, rhs: Vector3) {
        *self = Vector3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl ops::Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::SubAssign for Vector3 {
    fn sub_assign(&mut self, rhs: Vector3) {
        *self = Vector3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl ops::Div for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2],
        )
    }
}

impl ops::DivAssign for Vector3 {
    fn div_assign(&mut self, rhs: Vector3) {
        *self = Vector3::new(
            self.e[0] / rhs.e[0],
            self.e[1] / rhs.e[1],
            self.e[2] / rhs.e[2],
        )
    }
}

impl ops::Div<f32> for Vector3 {
    type Output = Vector3;

    fn div(self, rhs: f32) -> Vector3 {
        Vector3::new(
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs,
        )
    }
}

impl ops::DivAssign<f32> for Vector3 {
    fn div_assign(&mut self, rhs: f32) {
        *self = Vector3::new(
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs,
        )
    }
}

impl ops::Div<Vector3> for f32 {
    type Output = Vector3;

    fn div(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self / rhs.e[0],
            self / rhs.e[1],
            self / rhs.e[2],
        )
    }
}

impl ops::Mul for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::MulAssign for Vector3 {
    fn mul_assign(&mut self, rhs: Vector3) {
        *self = Vector3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl ops::Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, rhs: f32) -> Vector3 {
        Vector3::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs,
        )
    }
}

impl ops::MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, rhs: f32) {
        *self = Vector3::new(
            self.e[0] * rhs,
            self.e[1] * rhs,
            self.e[2] * rhs,
        )
    }
}

impl ops::Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, rhs: Vector3) -> Vector3 {
        Vector3::new(
            self * rhs.e[0],
            self * rhs.e[1],
            self * rhs.e[2],
        )
    }
}

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
