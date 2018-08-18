
use super::math::*;
use rand::{XorShiftRng, Rng, SeedableRng, distributions::Uniform};
use std::sync::Mutex;

const SEED: [u8; 16] = [1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16];

lazy_static! {
    static ref MATERIAL_RNG: Mutex<XorShiftRng> = Mutex::new(XorShiftRng::from_seed(SEED));
}

pub struct Bounce {
    pub attenuation: Vector3,
    pub bounced: Ray,
}

pub trait Material {
    fn scatter(&self, r: Ray, point: Vector3, normal: Vector3) -> Option<Bounce>;
}

pub struct Lambertian {
    albedo: Vector3,
}

impl Lambertian {
    pub fn new(albedo: Vector3) -> Lambertian {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r: Ray, point: Vector3, normal: Vector3) -> Option<Bounce> {
        let target = point + normal + Vector3::random_unit_sphere();
        let bounced = Ray::new(point, target - point);
        let attenuation = self.albedo;
        return Some(Bounce { attenuation, bounced })
    }
}


pub struct Metallic {
    albedo: Vector3,
    roughness: f32,
}

impl Metallic {
    pub fn new(albedo: Vector3, roughness: f32) -> Metallic {
        Metallic {
            albedo,
            roughness: if roughness < 1.0 { roughness } else { 1.0 },
        }
    }
}

impl Material for Metallic {
    fn scatter(&self, r: Ray, point: Vector3, normal: Vector3) -> Option<Bounce> {
        let target = r.direction.as_unit().reflect(normal);
        let bounced = Ray::new(point, target + self.roughness * Vector3::random_unit_sphere());
        let attenuation = self.albedo;
        if target.dot(normal) > 0.0 {
            Some(Bounce { attenuation, bounced })
        } else {
            None
        }
    }
}

pub struct Dielectric {
    refractive_index: f32
}

impl Dielectric {
    pub fn new(refractive_index: f32) -> Dielectric {
        Dielectric { refractive_index }
    }
}

fn schlick(cosine: f32, refractive_index: f32) -> f32 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
}

impl Material for Dielectric {
    fn scatter(&self, r: Ray, point: Vector3, normal: Vector3) -> Option<Bounce> {
        let reflection = r.direction.reflect(normal);
        let attenuation = Vector3::new(1.0, 1.0, 1.0);
        let (outward_normal, ni_over_nt, cosine) = if r.direction.dot(normal) > 0.0 {
            let cosine = self.refractive_index * r.direction.dot(normal) / r.direction.length();
            (-normal, self.refractive_index, cosine)
        } else {
            let cosine = -r.direction.dot(normal) / r.direction.length();
            (normal, 1.0 / self.refractive_index, cosine)
        };

        let (refraction, reflect_prob) = if let Some(refraction) = r.direction.refract(outward_normal, ni_over_nt) {
            (refraction, schlick(cosine, self.refractive_index))
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

        Some(Bounce { attenuation, bounced })
    }
}
