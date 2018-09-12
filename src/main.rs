extern crate image;
#[macro_use]
extern crate lazy_static;
extern crate minifb;
extern crate nfd;
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

use minifb::{Key, Menu, Scale, Window, WindowOptions, MENU_KEY_ALT, MENU_KEY_CTRL};
use nfd::Response;
use structopt::StructOpt;

const SCALE: Scale = Scale::X1;
const FILE_SAVE: usize = 1;
const FILE_QUIT: usize = 2;

#[derive(StructOpt, Debug)]
#[structopt(name = "PathTracer", about = "A simple ray tracer.")]
struct Args {
  /// Sets the width of the final rendered image.
  #[structopt(short = "w", long = "width", default_value = "400")]
  width: usize,
  /// Sets the height of the final rendered image.
  #[structopt(short = "h", long = "height", default_value = "300")]
  height: usize,
  /// Sets the count of samples taken per pixel.
  #[structopt(short = "s", long = "samples", default_value = "100")]
  samples: usize,
}

fn save_buffer_to_path(width: u32, height: u32, buffer: &[u32], path: &str) {
  println!("Writing {}x{} image to {}", width, height, path);
  let mut imgbuf = image::RgbImage::new(width, height);

  for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
    let index = (y * width + x) as usize;
    let value = buffer[index];
    let r = (value >> 16) & 0xFF;
    let g = (value >> 8) & 0xFF;
    let b = value & 0xFF;

    *pixel = image::Rgb([r as u8, g as u8, b as u8]);
  }

  imgbuf.save(path).unwrap();
}

fn main() {
  let args = Args::from_args();
  let width = args.width;
  let height = args.height;
  let samples = args.samples;

  let mut buffer: Option<Vec<u32>> = None;
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

  let mut file_menu = Menu::new("&File").unwrap();
  file_menu
    .add_item("Save", FILE_SAVE)
    .shortcut(Key::S, MENU_KEY_CTRL)
    .build();
  file_menu.add_separator();
  file_menu
    .add_item("Quit", FILE_QUIT)
    .shortcut(Key::F4, MENU_KEY_ALT)
    .build();

  window.add_menu(&file_menu);

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
  let mut running = true;
  while running {
    let (w, h) = window.get_size();
    let w = w / scale_factor;
    let h = h / scale_factor;
//    let pixels = w * h;

    running = window.is_open();
    window.is_menu_pressed().map(|menu_id| {
      match menu_id {
        FILE_SAVE => {
          if let Some(img) = buffer.take() {
            let result = nfd::open_save_dialog(Some("png"), None).unwrap_or_else(|e| {
              panic!(e);
            });

            match result {
              Response::Okay(path) => save_buffer_to_path(w as u32, h as u32, &img, &path),
              Response::OkayMultiple(paths) => save_buffer_to_path(w as u32, h as u32, &img, &paths[0]),
              Response::Cancel => (),
            }

            buffer = Some(img);
          }
        },
        FILE_QUIT => running = false,
        _ => (),
      };
    });

    if current_render_job.is_none() && buffer.is_none() {
      window.set_title(&format!("PathTracer - {}x{}", w, h));

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
      if let Ok(img) = rx.try_recv() {
        window.update_with_buffer(&img).unwrap();
        buffer = Some(img);
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
