#[macro_use]
extern crate lazy_static;
extern crate minifb;
extern crate rand;
extern crate rayon;
#[macro_use]
extern crate structopt;

mod camera;
mod geometry;
mod material;
mod math;
mod scene;

use camera::*;
use geometry::*;
use material::*;
use math::*;
use scene::*;

use std::{
  f32,
  sync::mpsc::{channel, Receiver},
  thread,
  time::Instant,
};

use minifb::{Key, Scale, Window, WindowOptions};
use structopt::StructOpt;

const SCALE: Scale = Scale::X1;

#[derive(StructOpt, Debug)]
#[structopt(name = "PathTracer", about = "A simple ray tracer.")]
struct Args {
  /// Sets the width of the final rendered image.
  #[structopt(short = "w", long = "width", default_value="400")]
  width: usize,
  /// Sets the height of the final rendered image.
  #[structopt(short = "h", long = "height", default_value="300")]
  height: usize,
  /// Sets the count of samples taken per pixel.
  #[structopt(short = "s", long = "samples", default_value="100")]
  samples: usize,
}

fn main() {
  let args = Args::from_args();
  let width = args.width;
  let height = args.height;
  let samples = args.samples;

  let mut buffer: Vec<u32> = vec![0; 0];
  let mut window = Window::new(
    "PathTracer",
    width,
    height,
    WindowOptions {
      resize: true,
      scale: SCALE,
      ..Default::default()
    },
  ).unwrap_or_else(|e| {
    panic!("{}", e);
  });

  let eye = Vector3::new(13.0, 2.0, 3.0);
  let look_at = Vector3::new(0.0, 0.0, 0.0);
  let focus_dist = 10.0;
  let aperture = 0.1;

  let scene = Scene::random();

  let scale_factor: usize = match SCALE {
    Scale::X1 => 1,
    Scale::X2 => 2,
    Scale::X4 => 4,
    Scale::X8 => 8,
    Scale::X16 => 16,
    Scale::X32 => 32,
    Scale::FitScreen => return,
  };

  let start = Instant::now();
  let mut current_render_job: Option<(Receiver<Vec<u32>>, thread::JoinHandle<_>)> = None;
  while window.is_open() && !window.is_key_down(Key::Escape) {
    let (w, h) = window.get_size();
    let w = w / scale_factor;
    let h = h / scale_factor;
    let pixels = w * h;
    if pixels != buffer.len() {
      window.set_title(&format!("PathTracer - {}x{}", w, h));

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

      let scene_copy = scene.clone();
      let (tx, rx) = channel();
      let handle = thread::spawn(move || {
        let buffer = scene_copy.render(camera, w, h, samples);
        tx.send(buffer).unwrap();
        println!("Render completed!");
      });

      current_render_job = Some((rx, handle));
    }

    if let Some((rx, handle)) = current_render_job.take() {
      if let Ok(buffer) = rx.try_recv() {
        window.update_with_buffer(&buffer).unwrap();
      } else {
        current_render_job = Some((rx, handle));
      }

      let delta = Instant::now() - start;
      let seconds = delta.as_secs() as f64 + (delta.subsec_millis() as f64 / 1000.0);
      window.set_title(&format!(
        "PathTracer - {}x{} - {}SPP - {:.2}s",
        w, h, samples, seconds
      ));
    }

    window.update();
  }
}
