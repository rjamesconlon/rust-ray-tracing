mod Vec;
use image::{ImageBuffer, Rgb, RgbImage};

fn generate_image() {
  const WIDTH: u32 = 256;
  const HEIGHT: u32 = 256;

  let mut buffer: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);

  let mut count = 0;
  let max = WIDTH * HEIGHT;

  for (x, y, pixel) in buffer.enumerate_pixels_mut() {
    println!("{count} out of {max}");
    let r = x as f64 / (WIDTH - 1) as f64;
    let g = y as f64 / (HEIGHT - 1) as f64;
    let b = 0.25;

    let ir = (255.999 * r) as u8;
    let ig = (255.999 * g) as u8;
    let ib = (255.999 * b) as u8;

    *pixel = Rgb([ir, ig, ib]);

    count += 1;
  }

  buffer.save("image.png").unwrap();

  println!("DONE!");
}

fn main() {
  // generate_image();
  let v1 = Vec::Vec::new(1.0, 2.0, 3.0);
  let v2 = Vec::Vec::new(10.0, 2.0, 3.0);

  let v3 = v1 + v2;

  println!(r#"v3: {0}, {1}, {2}"#, v3.x, v3.y, v3.z);
}
