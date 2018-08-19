#![allow(unused_imports)]

extern crate rand;
extern crate minifb;
#[macro_use]
extern crate lazy_static;
extern crate threadpool;
extern crate num_cpus;

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
use std::sync::mpsc::channel;
use threadpool::ThreadPool;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;
const SCALE: Scale = Scale::X1;

fn main() {

    let mut buffer: Vec<u32> = vec![0; 0];
    let mut window = Window::new("PathTracer - Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions {
                                     resize: true,
                                     scale: SCALE,
                                     .. Default::default()
                                 }).unwrap_or_else(|e| { panic!("{}", e); });



    let eye = Vector3::new(13.0, 2.0, 3.0);
    let look_at = Vector3::new(0.0, 0.0, 0.0);
    let focus_dist = 10.0;
    let aperture = 0.1;

    let scene = Scene::random();
    let pool = ThreadPool::new(num_cpus::get());

    let mut scene_chan = channel();
    let scale_factor: usize = match SCALE {
        Scale::X1 => 1,
        Scale::X2 => 2,
        Scale::X4 => 4,
        Scale::X8 => 8,
        Scale::X16 => 16,
        Scale::X32 => 32,
        Scale::FitScreen => return,
    };


    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (w, h) = window.get_size();
        let w = w / scale_factor;
        let h = h / scale_factor;
        // get_unscaled_size(&window);
        let pixels = w * h;
        if pixels != buffer.len() {
            window.set_title(&format!("PathTracer ({}x{}) - Rendering...", w, h));
            drop(scene_chan.1);
            scene_chan = channel();

            buffer.clear();
            buffer.resize(pixels, 0);

            let camera = Camera::new(
                eye,
                look_at,
                Vector3::up(),
                20.0,
                w as f32 / h as f32,
                aperture,
                focus_dist,
            );
            scene.render(camera, w, h, &pool, scene_chan.0);
        }

        if let Ok(data) = scene_chan.1.try_recv() {
            buffer[data.index] = data.value;
        } else {
            window.set_title(&format!("PathTracer ({}x{})", w, h));
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }

    drop(scene_chan.1);
    pool.join();
}
