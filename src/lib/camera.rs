use crate::lib::{colour, hittable, interval, ray, utility, vector};
use image::{ImageBuffer, Rgb, RgbImage};

pub struct Camera {
  // aspect ratio and plane size
  aspect_ratio: f64,
  image_width: usize,
  image_height: usize,

  camera_center: vector::Vector,
  pixel00_loc: vector::Vector,
  pixel_delta_u: vector::Vector,
  pixel_delta_v: vector::Vector,
}

impl Default for Camera {
  // Constructor that initializes the camera with image width and aspect ratio.
  fn default() -> Self {
    let mut camera = Camera {
      aspect_ratio: 16.0 / 9.0,
      image_width: 400,
      image_height: 0,
      camera_center: vector::Vector::new(0.0, 0.0, 0.0),
      pixel00_loc: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_u: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_v: vector::Vector::new(0.0, 0.0, 0.0),
    };

    // Initialize computed fields
    camera.initialize();

    camera
  }
}

impl Camera {
  // Constructor that initializes the camera with image width and aspect ratio.
  pub fn new(aspect_ratio: f64, image_width: usize) -> Self {
    let mut camera = Camera {
      aspect_ratio,
      image_width,
      image_height: 0,
      camera_center: vector::Vector::new(0.0, 0.0, 0.0),
      pixel00_loc: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_u: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_v: vector::Vector::new(0.0, 0.0, 0.0),
    };

    // Initialize computed fields
    camera.initialize();

    camera
  }

  fn initialize(&mut self) {
    self.image_height = (self.image_width as f64 / self.aspect_ratio) as usize;
    self.image_height = self.image_height.max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

    // Calculate vectors across the horizontal and vertical viewport edges
    let viewport_u = vector::Vector::new(viewport_width, 0.0, 0.0);
    let viewport_v = vector::Vector::new(0.0, -viewport_height, 0.0);

    // Calculate horizontal and vertical delta vectors
    self.pixel_delta_u = viewport_u / self.image_width as f64;
    self.pixel_delta_v = viewport_v / self.image_height as f64;

    // Calculate the location of the upper left pixel
    let viewport_upper_left = self.camera_center
      - vector::Vector::new(0.0, 0.0, focal_length)
      - viewport_u / 2.0
      - viewport_v / 2.0;
    self.pixel00_loc = viewport_upper_left + ((self.pixel_delta_u + self.pixel_delta_v) * 0.5);
  }
}

impl Camera {
  pub fn render(&self, world: impl hittable::Hittable) {
    let mut count = 0;
    let max = self.image_width * self.image_height;

    let mut buffer: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
      println!("{count} out of {max}: {x} {y}");

      let pixel_center =
        self.pixel00_loc + (self.pixel_delta_u * x as f64) + (self.pixel_delta_v * y as f64);
      let ray_direction = pixel_center - self.camera_center;
      // dbg!(pixel_center);
      // dbg!(camera_center);
      // dbg!(ray_direction);
      let ray: ray::Ray = ray::Ray::new(self.camera_center, ray_direction);

      let (r, g, b) = ray.ray_colour(&world);
      *pixel = Rgb([r, g, b]);

      count += 1;
    }

    buffer.save("img.png").unwrap();

    println!("DONE!");
  }

  pub fn ray_colour(r: &ray::Ray, world: &dyn hittable::Hittable) -> (u8, u8, u8) {
    let mut hit_rec = hittable::HitRecord::new(
      vector::Vector::new(0.0, 0.0, 0.0),
      vector::Vector::new(0.0, 0.0, 0.0),
      0.0,
      false,
    );
    if world.hit(
      r,
      interval::Interval::new(0.0, utility::INFINITY),
      &mut hit_rec,
    ) {
      return colour::get_colour(&((hit_rec.normal + vector::Vector::new(1.0, 1.0, 1.0)) * 0.5));
    } else {
      let unit_dir = vector::Vector::unit_vector(&r.dir);
      let a = (unit_dir.y + 1.0) * 0.5;
      colour::get_colour(&{
        vector::Vector::new(1.0, 1.0, 1.0) * (1.0 - a) + vector::Vector::new(0.5, 0.7, 1.0) * a
      })
    }
  }
}
