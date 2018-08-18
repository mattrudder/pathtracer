#![allow(unused_imports)]

extern crate rand;
extern crate minifb;
#[macro_use]
extern crate lazy_static;
extern crate rayon;

mod camera;
mod geometry;
mod math;
mod material;
mod scene;

use camera::*;
use geometry::*;
use math::*;
use material::*;
use scene::*;

use std::time::Instant;
use std::f32;
use std::sync::Arc;
use rand::{XorShiftRng, Rng, SeedableRng, distributions::Uniform};
use minifb::{Key, KeyRepeat, Window, WindowOptions, Scale};
use std::sync::Mutex;
use std::thread;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("PathTracer - Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                    .. Default::default()
                                 }).unwrap_or_else(|e| { panic!("{}", e); });

    let eye = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;
    let camera = Camera::new(
        eye,
        look_at,
        Vector3::up(),
        20.0,
        WIDTH as f32 / HEIGHT as f32,
        aperture,
        focus_dist,
    );

    let mut scene = Scene::random(camera);
    let data = Arc::new(Mutex::new(buffer));

    let data2 = data.clone();
    let handle = thread::spawn(move || scene.render(WIDTH, HEIGHT, data2));

    let mut last = Instant::now();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let current = Instant::now();
        let delta = current.duration_since(last);
        last = current;

        window.set_title(&format!("PathTracer - Press ESC to exit"));

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        let buffer = data.lock().unwrap();
        window.update_with_buffer(&buffer).unwrap();
    }

    handle.join().unwrap();
}
