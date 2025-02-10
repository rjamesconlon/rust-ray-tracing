use image::{ImageBuffer, Rgb, RgbImage};

fn generate_image() {
  const WIDTH: u32 = 256;
  const HEIGHT: u32 = 256;

  let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    let r = x as f64 / (WIDTH - 1) as f64;
    let g = y as f64 / (HEIGHT - 1) as f64;
    let b = 0.25;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;

    *pixel = Rgb([ir, ig, ib]);
  }

  buffer.save("image.png").unwrap();
}

fn main() {
  generate_image();
}
