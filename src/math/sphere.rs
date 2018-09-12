use super::*;
use std::f32;

#[derive(Debug, Clone, Copy)]
pub struct Sphere {
  pub center: Vector3,
  pub radius: f32,
}

impl Sphere {
  pub fn new(center: Vector3, radius: f32) -> Sphere {
    Sphere { center, radius }
  }
}

impl Collidable<Ray> for Sphere {
  type Output = Option<RayHit>;

  fn hit(&self, r: Ray) -> Option<RayHit> {
    let oc = r.origin - self.center;
    let a = r.direction.dot(r.direction);
    let b = oc.dot(r.direction);
    let c = oc.dot(oc) - self.radius * self.radius;

    let discriminant = b.powi(2) - a * c;
    if discriminant < 0.0 {
      return None;
    }

    let d = (b * b - (a * c)).sqrt();
    let t = (-b - d) / a;
    if t < f32::MAX && t > 0.001 {
      let point = r.point_at_parameter(t);
      let normal = (point - self.center) / self.radius;
      Some(RayHit { t, point, normal })
    } else {
      let t = (-b - d) / a;
      if t < f32::MAX && t > 0.001 {
        let point = r.point_at_parameter(t);
        let normal = (point - self.center) / self.radius;
        Some(RayHit { t, point, normal })
      } else {
        None
      }
    }
  }
}
