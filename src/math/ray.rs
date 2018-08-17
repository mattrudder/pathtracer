use super::*;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vector3,
    pub direction: Vector3,
}

impl Ray {
    pub fn new(origin: Vector3, direction: Vector3) -> Ray {
        Ray { origin, direction }
    }

    pub fn point_at_parameter(self, t: f32) -> Vector3 {
        self.origin + t * self.direction
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RayHit {
    pub t: f32,
    pub point: Vector3,
    pub normal: Vector3,
}

pub trait Collidable<TPrimitive> {
    type Output;

    fn hit(&self, r: TPrimitive) -> Self::Output;
}
