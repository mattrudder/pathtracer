#![allow(unused_imports)]

extern crate rand;
extern crate minifb;

mod math;

use minifb::{Key, KeyRepeat, Window, WindowOptions, Scale};
use math::*;
use std::time::Instant;

use rand::{XorShiftRng, Rng, SeedableRng};
use rand::distributions::Uniform;


const WIDTH: usize = 400;
const HEIGHT: usize = 200;
const SAMPLES: usize = 100;

//fn hit_sphere(center: Vector3, radius: f32, r: Ray) -> bool {
//    let oc = r.origin - center;
//    let a = r.direction.dot(r.direction);
//    let b = 2.0 * oc.dot(r.direction);
//    let c = oc.dot(oc) - radius * radius;
//    let discriminant = b * b - 4.0 * a * c;
//    discriminant > 0.0
//}

struct Scene {
    pub items: Vec<Box<dyn RayHitable>>,
}

impl Scene {
    pub fn new(items: Vec<Box<dyn RayHitable>>) -> Scene {
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

impl RayHitable for Scene {
    fn ray_hit(&self, r: Ray, t_min: f32, t_max: f32) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut t = t_max;
        for item in self.items.iter() {
            if let Some(hit) = item.ray_hit(r, t_min, t) {
                t = hit.t;
                result = Some(hit)
            }
        }

        result
    }
}

fn color<T: RayHitable>(r: Ray, item: &T) -> Vector3 {
    if let Some(hit) = item.ray_hit(r, 0.0, std::f32::MAX) {
        0.5 * Vector3::new(hit.normal.x() + 1.0, hit.normal.y() + 1.0, hit.normal.z() + 1.0)
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
                                     scale: Scale::X4,
                                    .. Default::default()
                                 }).unwrap_or_else(|e| { panic!("{}", e); });



    let scene = Scene::new(vec![
        Box::new(Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0)),
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

        let seed = [1,2,3,4, 5,6,7,8, 9,10,11,12, 13,14,15,16];
        let mut rng = XorShiftRng::from_seed(seed);
        let uniform = Uniform::new(0.0f32, 1.0f32);

        for (row, stride) in buffer.chunks_mut(WIDTH).enumerate() {
            for (col, pixel) in stride.iter_mut().enumerate() {
                let mut c = Vector3::zero();
                for sample in 0..SAMPLES {
                    let u = (col as f32 + rng.sample(uniform)) / WIDTH as f32;
                    let v = ((HEIGHT - row) as f32 + rng.sample(uniform)) / HEIGHT as f32;

                    let ray = cam.get_ray(u, v);
                    c = c + color(ray, &scene);


                }

                *pixel = (c / SAMPLES as f32).to_rgb24();
            }
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
