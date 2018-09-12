use super::{Ray, Vector3};

use std::f32;

#[derive(Copy, Clone, Debug)]
pub struct Camera {
  eye: Vector3,
  lower_left_corner: Vector3,
  horizontal: Vector3,
  vertical: Vector3,
  lens_radius: f32,
  x: Vector3,
  y: Vector3,
  z: Vector3,
}

impl Camera {
  pub fn new(
    eye: Vector3,
    target: Vector3,
    up: Vector3,
    fov: f32,
    aspect: f32,
    aperture: f32,
    focus_dist: f32,
  ) -> Camera {
    let theta = fov * f32::consts::PI / 180.0;
    let half_height = (theta * 0.5).tan();
    let half_width = aspect * half_height;

    let z = (eye - target).as_unit();
    let x = up.cross(z).as_unit();
    let y = z.cross(x);

    Camera {
      eye,
      x,
      y,
      z,
      lens_radius: aperture * 0.5,
      lower_left_corner: eye
        - half_width * focus_dist * x
        - half_height * focus_dist * y
        - focus_dist * z,
      horizontal: 2.0 * half_width * focus_dist * x,
      vertical: 2.0 * half_height * focus_dist * y,
    }
  }

  pub fn get_ray(&self, u: f32, v: f32) -> Ray {
    let rd = self.lens_radius * Vector3::random_unit_disk();
    let offset = self.x * rd.x() + self.y * rd.y();
    Ray::new(
      self.eye + offset,
      self.lower_left_corner + u * self.horizontal + v * self.vertical - self.eye - offset,
    )
  }
}
