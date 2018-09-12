use super::{Camera, Collidable, Geometry, Material, Ray, RayHit, Sphere, Vector3};

use std::{
  f32,
  sync::mpsc::Sender,
  time::{Duration, Instant},
};

use rand::{distributions::Uniform, Rng, SeedableRng, XorShiftRng};
use rayon::prelude::*;

const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

pub trait SceneItem: Collidable<Ray> {
  fn get_material(&self) -> Material;
}

#[derive(Clone)]
pub struct Scene {
  pub items: Vec<Geometry>,
  pub is_dirty: bool,
}

impl Scene {
  #[allow(dead_code)]
  pub fn new(items: Vec<Geometry>) -> Scene {
    Scene {
      items,
      is_dirty: true,
    }
  }

  pub fn random() -> Scene {
    let mut items = vec![];

    let mut rng = XorShiftRng::from_seed(SEED);
    let dist = Uniform::new(0.0f32, 1.0f32);

    items.push(Geometry::from_sphere(
      Sphere::new(Vector3::new(0.0, -1000.0, 0.0), 1000.0),
      Material::lambert(Vector3::new(0.5, 0.5, 0.5)),
    ));

    for a in -11..12 {
      for b in -11..12 {
        let choose_mat = rng.sample(dist);
        let center = Vector3::new(
          a as f32 + 0.9 * rng.sample(dist),
          0.2,
          b as f32 + 0.9 * rng.sample(dist),
        );
        if (center - Vector3::new(4.0, 0.2, 0.0)).length() > 0.9 {
          let material = if choose_mat < 0.8 {
            Material::lambert(Vector3::new(
              rng.sample(dist) * rng.sample(dist),
              rng.sample(dist) * rng.sample(dist),
              rng.sample(dist) * rng.sample(dist),
            ))
          } else if choose_mat < 0.95 {
            Material::metal(
              Vector3::new(
                0.5 * (1.0 + rng.sample(dist)),
                0.5 * (1.0 + rng.sample(dist)),
                0.5 * (1.0 + rng.sample(dist)),
              ),
              0.5 * rng.sample(dist),
            )
          } else {
            Material::dielectric(1.5)
          };

          items.push(Geometry::from_sphere(Sphere::new(center, 0.2), material));
        }
      }
    }

    items.push(Geometry::from_sphere(
      Sphere::new(Vector3::new(0.0, 1.0, 0.0), 1.0),
      Material::dielectric(1.5),
    ));

    items.push(Geometry::from_sphere(
      Sphere::new(Vector3::new(-4.0, 1.0, 0.0), 1.0),
      Material::lambert(Vector3::new(0.4, 0.2, 0.1)),
    ));

    items.push(Geometry::from_sphere(
      Sphere::new(Vector3::new(4.0, 1.0, 0.0), 1.0),
      Material::metal(Vector3::new(0.7, 0.6, 0.5), 0.1),
    ));

    Scene {
      items,
      is_dirty: true,
    }
  }

  fn color(r: Ray, scene: &Scene, depth: u32) -> Vector3 {
    if let Some(hit) = scene.hit(r) {
      if depth < 50 {
        let material = hit.item.get_material();
        if let Some(bounce) = material.scatter(r, hit.hit.point, hit.hit.normal) {
          bounce.attenuation * Scene::color(bounce.bounced, scene, depth + 1)
        } else {
          Vector3::zero()
        }
      } else {
        Vector3::zero()
      }
    } else {
      let direction = r.direction.as_unit();
      let t = 0.5 * (direction.y() + 1.0);
      Vector3::lerp(t, Vector3::one(), Vector3::new(0.5, 0.7, 1.0))
    }
  }

  pub fn render(&self, camera: Camera, width: usize, height: usize, samples: usize) -> Vec<u32> {
    let dist = Uniform::new(0.0f32, 1.0f32);

    let mut buffer = vec![0; width * height];
    buffer
      .par_chunks_mut(width)
      .enumerate()
      .for_each(|(row, line)| {
        for col in 0..width {
          let mut c = Vector3::zero();
          let mut rng = XorShiftRng::from_seed(SEED);
          for _ in 0..samples {
            let u = (col as f32 + rng.sample(dist)) / width as f32;
            let v = ((height - row) as f32 + rng.sample(dist)) / height as f32;

            let ray = camera.get_ray(u, v);
            c += Scene::color(ray, &self, 0);
          }

          c /= samples as f32;

          // gamma 2 adjustment
          c = Vector3::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

          line[col] = c.to_rgb24();
        }
      });

    return buffer;
  }
}

pub struct SceneRayHit {
  hit: RayHit,
  item: Geometry,
}

impl Collidable<Ray> for Scene {
  type Output = Option<SceneRayHit>;

  fn hit(&self, r: Ray) -> Option<SceneRayHit> {
    let mut result: Option<SceneRayHit> = None;
    let mut t = f32::MAX;

    for item in self.items.iter() {
      if let Some(hit) = item.hit(r) {
        if hit.t < t {
          t = hit.t;
          result = Some(SceneRayHit { hit, item: *item })
        }
      }
    }

    result
  }
}

impl SceneItem for Scene {
  fn get_material(&self) -> Material {
    Material::lambert(Vector3::zero())
  }
}
