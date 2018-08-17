#![allow(unused_imports)]

extern crate rand;
extern crate minifb;
#[macro_use]
extern crate lazy_static;

mod geometry;
mod math;
mod material;

use minifb::{Key, KeyRepeat, Window, WindowOptions, Scale};
use math::*;
use geometry::*;
use material::*;
use std::time::Instant;
use std::f32;
use std::sync::Arc;

use rand::{XorShiftRng, Rng, SeedableRng, distributions::Uniform};

const WIDTH: usize = 400;
const HEIGHT: usize = 200;
const SAMPLES: usize = 100;
const SEED: [u8; 16] = [1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16];

struct Scene {
    pub items: Vec<Arc<dyn SceneItem<Output=Option<RayHit>>>>,
}

impl Scene {
    pub fn new(items: Vec<Arc<dyn SceneItem<Output=Option<RayHit>>>>) -> Scene {
        Scene { items }
    }
}

struct Camera {
    lower_left_corner: Vector3,
    origin: Vector3,
    horizontal: Vector3,
    vertical: Vector3,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Vector3::new(-2.0, -1.0, -1.0),
            horizontal: Vector3::new(4.0, 0.0, 0.0),
            vertical: Vector3::new(0.0, 2.0, 0.0),
            origin:  Vector3::zero(),
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical)
    }
}

struct SceneRayHit {
    hit: RayHit,
    item: Arc<SceneItem<Output=Option<RayHit>>>
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
                    result = Some(SceneRayHit { hit, item: item.clone() })
                }
            }
        }

        result
    }
}

impl SceneItem for Scene {
    fn get_material(&self) -> Arc<dyn Material> {
        Arc::new(Lambertian::new(Vector3::zero()))
    }
}

fn color(r: Ray, scene: &Scene, depth: u32) -> Vector3 {
    if let Some(hit) = scene.hit(r) {
        if depth < 50 {
            let material = hit.item.get_material();
            if let Some(bounce) = material.scatter(r, hit.hit.point, hit.hit.normal) {
                bounce.attenuation * color(bounce.bounced, scene, depth + 1)
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

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("PathTracer - Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                     scale: Scale::X2,
                                    .. Default::default()
                                 }).unwrap_or_else(|e| { panic!("{}", e); });

    let scene = Scene::new(vec![
        Arc::new(
            SphereGeometry::new(
                Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
                Arc::new(Lambertian::new(Vector3::new(0.8, 0.3, 0.3))),
            )
        ),
        Arc::new(
            SphereGeometry::new(
                Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
                Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
            )
        ),
        Arc::new(
            SphereGeometry::new(
                Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5),
                Arc::new(Metallic::new(Vector3::new(0.8, 0.6, 0.2), 1.0)),
            )
        ),
        Arc::new(
            SphereGeometry::new(
                Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5),
                Arc::new(Metallic::new(Vector3::new(0.8, 0.8, 0.8), 0.3)),
            )
        ),
    ]);

    let cam = Camera::new();

    let mut last = Instant::now();
    let mut total: f64 = 0.0;
    let mut second: f64 = 0.0;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current = Instant::now();
        let delta = current.duration_since(last);
        last = current;

        let dt = delta.as_secs() as f64 + delta.subsec_nanos() as f64 * 1e-9;
        total += dt;
        second += dt;

        window.set_title(&format!("PathTracer - Press ESC to exit [{}ms/f]", delta.subsec_millis()));

        let mut rng = XorShiftRng::from_seed(SEED);
        let dist = Uniform::new(0.0f32, 1.0f32);

        for (row, stride) in buffer.chunks_mut(WIDTH).enumerate() {
            for (col, pixel) in stride.iter_mut().enumerate() {
                let mut c = Vector3::zero();
                for _ in 0..SAMPLES {
                    let u = (col as f32 + rng.sample(dist)) / WIDTH as f32;
                    let v = ((HEIGHT - row) as f32 + rng.sample(dist)) / HEIGHT as f32;

                    let ray = cam.get_ray(u, v);
                    c += color(ray, &scene, 0);
                }

                c /= SAMPLES as f32;

                // gamma 2 adjustment
                c = Vector3::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

                *pixel = c.to_rgb24();
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
