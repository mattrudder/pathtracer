#![allow(dead_code)]

use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vector3 {
    e: [f32; 3]
}

impl Vector3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vector3 {
        Vector3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f32 { return self.e[0]; }
    pub fn y(&self) -> f32 { return self.e[1]; }
    pub fn z(&self) -> f32 { return self.e[2]; }
    pub fn r(&self) -> f32 { return self.e[0]; }
    pub fn g(&self) -> f32 { return self.e[1]; }
    pub fn b(&self) -> f32 { return self.e[2]; }

    pub fn length_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    pub fn normalize(&mut self) {
        let k = 1.0 / self.length();
        self.e[0] *= k;
        self.e[1] *= k;
        self.e[2] *= k;
    }

    pub fn as_unit(&self) -> Vector3 {
        let k = 1.0 / self.length();
        Vector3::new(
            self.e[0] * k,
            self.e[1] * k,
            self.e[2] * k,
        )
    }

    pub fn dot(&self, rhs: &Vector3) -> f32 {
        self.e[0] * rhs.e[0] + self.e[1] * rhs.e[1] + self.e[2] * rhs.e[2]
    }

    pub fn cross(&self, rhs: &Vector3) -> Vector3 {
        Vector3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[0] * rhs.e[2] - self.e[2] * rhs.e[0],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0]
        )
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

impl fmt::Display for Vector3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}
