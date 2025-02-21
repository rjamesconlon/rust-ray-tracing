mod colour;
mod hittable;
mod hittable_list;
mod ray;
mod sphere;
mod vector;
use image::{ImageBuffer, Rgb, RgbImage};

// img dimensions
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const WIDTH: usize = 400;
const HEIGHT: usize = (WIDTH as f64 / ASPECT_RATIO) as usize;

// viewport dimensions and camera
const FOCAL_LENGTH: f64 = 1.0;
const VIEWPORT_HEIGHT: f64 = 2.0;
const VIEWPORT_WIDTH: f64 = VIEWPORT_HEIGHT * (WIDTH as f64 / HEIGHT as f64);

fn main() {
  let camera_center: vector::Vector = vector::Vector {
    x: 0.0,
    y: 0.0,
    z: 0.0,
  };
  // make viewport full vectors
  let viewport_u: vector::Vector = vector::Vector::new(VIEWPORT_WIDTH, 0.0, 0.0);
  let viewport_v: vector::Vector = vector::Vector::new(0.0, -VIEWPORT_HEIGHT, 0.0);

  // make delta vectors for u,v
  let pixel_delta_u = viewport_u / WIDTH as f64;
  let pixel_delta_v = viewport_v / HEIGHT as f64;
  // location of upper left pixel
  let viewport_upper_left = camera_center
    - vector::Vector::new(0.0, 0.0, FOCAL_LENGTH)
    - (viewport_u / 2.0)
    - (viewport_v / 2.0);
  let pixel00_loc = viewport_upper_left + ((pixel_delta_u + pixel_delta_v) * 0.5);
  // make image buffer
  let mut buffer: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

  // progress counting
  let mut count = 0;
  let max = WIDTH * HEIGHT;

  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    // println!("{count} out of {max}: {x} {y}");

    let pixel_center = pixel00_loc + (pixel_delta_u * x as f64) + (pixel_delta_v * y as f64);
    let ray_direction = pixel_center - camera_center;
    // dbg!(pixel_center);
    // dbg!(camera_center);
    // dbg!(ray_direction);
    let ray: ray::Ray = ray::Ray::new(camera_center, ray_direction);

    let (r, g, b) = ray.ray_colour();
    *pixel = Rgb([r, g, b]);

    count += 1;
  }

  buffer.save("image2.png").unwrap();

  println!("DONE!");
}
