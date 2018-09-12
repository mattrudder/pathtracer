use super::math::*;

use std::sync::Mutex;

use rand::{distributions::Uniform, Rng, SeedableRng, XorShiftRng};

const SEED: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];

lazy_static! {
  static ref MATERIAL_RNG: Mutex<XorShiftRng> = Mutex::new(XorShiftRng::from_seed(SEED));
}

#[derive(Debug, Clone, Copy)]
pub struct Bounce {
  pub attenuation: Vector3,
  pub bounced: Ray,
}

#[derive(Debug, Clone, Copy)]
pub enum Material {
  Lambertian { albedo: Vector3 },
  Metallic { albedo: Vector3, roughness: f32 },
  Dielectric { refractive_index: f32 },
}

fn lambertian_scatter(_: Ray, point: Vector3, normal: Vector3, albedo: Vector3) -> Option<Bounce> {
  let target = point + normal + Vector3::random_unit_sphere();
  let bounced = Ray::new(point, target - point);
  let attenuation = albedo;
  return Some(Bounce {
    attenuation,
    bounced,
  });
}

fn metallic_scatter(
  r: Ray,
  point: Vector3,
  normal: Vector3,
  albedo: Vector3,
  roughness: f32,
) -> Option<Bounce> {
  let target = r.direction.as_unit().reflect(normal);
  let bounced = Ray::new(point, target + roughness * Vector3::random_unit_sphere());
  let attenuation = albedo;
  if target.dot(normal) > 0.0 {
    Some(Bounce {
      attenuation,
      bounced,
    })
  } else {
    None
  }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
  let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
  let r0 = r0 * r0;
  r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}

fn dielectric_scatter(
  r: Ray,
  point: Vector3,
  normal: Vector3,
  refractive_index: f32,
) -> Option<Bounce> {
  let reflection = r.direction.reflect(normal);
  let attenuation = Vector3::new(1.0, 1.0, 1.0);
  let (outward_normal, ni_over_nt, cosine) = if r.direction.dot(normal) > 0.0 {
    let cosine = refractive_index * r.direction.dot(normal) / r.direction.length();
    (-normal, refractive_index, cosine)
  } else {
    let cosine = -r.direction.dot(normal) / r.direction.length();
    (normal, 1.0 / refractive_index, cosine)
  };

  let (refraction, reflect_prob) =
    if let Some(refraction) = r.direction.refract(outward_normal, ni_over_nt) {
      (refraction, schlick(cosine, refractive_index))
    } else {
      (Vector3::zero(), 1.0)
    };

  let uniform = Uniform::new(0.0f32, 1.0f32);
  let mut rng = MATERIAL_RNG.lock().unwrap();
  let bounced = if rng.sample(uniform) < reflect_prob {
    Ray::new(point, reflection)
  } else {
    Ray::new(point, refraction)
  };

  Some(Bounce {
    attenuation,
    bounced,
  })
}

impl Material {
  pub fn lambert(albedo: Vector3) -> Material {
    Material::Lambertian { albedo }
  }

  pub fn metal(albedo: Vector3, roughness: f32) -> Material {
    let roughness = if roughness < 1.0 { roughness } else { 1.0 };
    Material::Metallic { albedo, roughness }
  }

  pub fn dielectric(refractive_index: f32) -> Material {
    Material::Dielectric { refractive_index }
  }

  pub fn scatter(&self, r: Ray, point: Vector3, normal: Vector3) -> Option<Bounce> {
    match self {
      Material::Lambertian { albedo } => lambertian_scatter(r, point, normal, *albedo),
      Material::Metallic { albedo, roughness } => {
        metallic_scatter(r, point, normal, *albedo, *roughness)
      },
      Material::Dielectric { refractive_index } => {
        dielectric_scatter(r, point, normal, *refractive_index)
      },
    }
  }
}
