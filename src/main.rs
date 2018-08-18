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

struct Camera {
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
    pub fn new(eye: Vector3, target: Vector3, up: Vector3, fov: f32, aspect: f32, aperture: f32, focus_dist: f32) -> Camera {
        let theta = fov * f32::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect * half_height;

        let z = (eye - target).as_unit();
        let x = up.cross(z).as_unit();
        let y = z.cross(x);

        Camera {
            eye,
            x, y, z,
            lens_radius: aperture * 0.5,
            lower_left_corner: eye - half_width * focus_dist * x - half_height * focus_dist * y - focus_dist * z,
            horizontal: 2.0 * half_width * focus_dist * x,
            vertical: 2.0 * half_height * focus_dist * y,

        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let rd = self.lens_radius * Vector3::random_unit_disk();
        let offset = self.x * rd.x() + self.y * rd.y();
        Ray::new(self.eye + offset, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.eye - offset)
    }
}

struct Scene {
    pub items: Vec<Arc<dyn SceneItem<Output=Option<RayHit>>>>,
    pub camera: Camera,
    pub is_dirty: bool,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        Scene { camera, items: vec![], is_dirty: true }
    }

    pub fn update(&mut self) {
        self.is_dirty = true;
    }

    pub fn push<T: 'static + Sized + SceneItem<Output=Option<RayHit>>>(&mut self, item: T) {
        self.items.push(Arc::new(item));
        self.is_dirty = true;
    }

    pub fn render(&mut self, buffer: &mut [u32]) {
        if !self.is_dirty {
            return;
        }

        let mut rng = XorShiftRng::from_seed(SEED);
        let dist = Uniform::new(0.0f32, 1.0f32);

        for (row, stride) in buffer.chunks_mut(WIDTH).enumerate() {
            for (col, pixel) in stride.iter_mut().enumerate() {
                let mut c = Vector3::zero();
                for _ in 0..SAMPLES {
                    let u = (col as f32 + rng.sample(dist)) / WIDTH as f32;
                    let v = ((HEIGHT - row) as f32 + rng.sample(dist)) / HEIGHT as f32;

                    let ray = self.camera.get_ray(u, v);
                    c += color(ray, self, 0);
                }

                c /= SAMPLES as f32;

                // gamma 2 adjustment
                c = Vector3::new(c.r().sqrt(), c.g().sqrt(), c.b().sqrt());

                *pixel = c.to_rgb24();
            }
        }

        self.is_dirty = false;
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
//                                     scale: Scale::X2,
                                    .. Default::default()
                                 }).unwrap_or_else(|e| { panic!("{}", e); });

    let eye = Vector3::new(-2.0, 2.0, 1.0);
    let look_at = Vector3::new(0.0, 0.0, -1.0);
    let focus_dist = (eye - look_at).length();
    let aperture = 0.0;
    let camera = Camera::new(
        eye,
        look_at,
        Vector3::up(),
        60.0,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        focus_dist,
    );
    let mut scene = Scene::new(camera);
    scene.push(SphereGeometry::new(
        Sphere::new(Vector3::new(0.0, 0.0, -1.0), 0.5),
        Arc::new(Lambertian::new(Vector3::new(0.1, 0.2, 0.5))),
    ));
    scene.push(SphereGeometry::new(
        Sphere::new(Vector3::new(0.0, -100.5, -1.0), 100.0),
        Arc::new(Lambertian::new(Vector3::new(0.8, 0.8, 0.0))),
    ));
    scene.push(SphereGeometry::new(
        Sphere::new(Vector3::new(1.0, 0.0, -1.0), 0.5),
        Arc::new(Metallic::new(Vector3::new(0.8, 0.6, 0.2), 0.2)),
    ));
    scene.push(SphereGeometry::new(
        Sphere::new(Vector3::new(-1.0, 0.0, -1.0), 0.5),
        Arc::new(Dielectric::new(1.5)),
    ));
    scene.push(SphereGeometry::new(
        Sphere::new(Vector3::new(-1.0, 0.0, -1.0), -0.45),
        Arc::new(Dielectric::new(1.5)),
    ));

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

        scene.render(&mut buffer);

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
