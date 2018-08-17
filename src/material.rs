
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
        let mut rng = MATERIAL_RNG.lock().unwrap();
        let target = point + normal + Vector3::random(&mut *rng);
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
        let mut rng = MATERIAL_RNG.lock().unwrap();
        let target = r.direction.as_unit().reflect(normal);
        let bounced = Ray::new(point, target + self.roughness * Vector3::random(&mut *rng));
        let attenuation = self.albedo;
        if target.dot(normal) > 0.0 {
            Some(Bounce { attenuation, bounced })
        } else {
            None
        }
    }
}
