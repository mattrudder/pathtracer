#![allow(unused_imports)]

extern crate minifb;

mod math;

use minifb::{Key, Window, WindowOptions};
use math::*;


const WIDTH: usize = 800;
const HEIGHT: usize = 600;

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new("PathTracer - Press ESC to exit",
                                 WIDTH,
                                 HEIGHT,
                                 WindowOptions::default()).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for (n, i) in buffer.iter_mut().enumerate() {
            let row = (n / WIDTH) as f32;
            let col = (n % WIDTH) as f32;
            let u = col / WIDTH as f32;
            let v = row / HEIGHT as f32;
            let color = Vector3::new(u, v, 0.2);

            let r = (color.r() * 255.99f32).trunc() as u32;
            let g = (color.g() * 255.99f32).trunc() as u32;
            let b = (color.b() * 255.99f32).trunc() as u32;
            *i = (r << 16) | (g << 8) | b;
        }

        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        window.update_with_buffer(&buffer).unwrap();
    }
}
