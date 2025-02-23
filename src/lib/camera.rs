use crate::lib::{colour, hittable, interval, ray, utility, vector};
use image::{ImageBuffer, Rgb, RgbImage};

pub struct Camera {
  // aspect ratio and plane size
  aspect_ratio: f64,
  image_width: usize,
  image_height: usize,
  samples_per_pixel: u32,

  camera_center: vector::Vector,
  pixel00_loc: vector::Vector,
  pixel_delta_u: vector::Vector,
  pixel_delta_v: vector::Vector,
  pixel_samples_scale: f64,
  max_depth: u32,
}

impl Default for Camera {
  // Constructor that initializes the camera with image width and aspect ratio.
  fn default() -> Self {
    let mut camera = Camera {
      aspect_ratio: 16.0 / 9.0,
      image_width: 400,
      image_height: 0,
      samples_per_pixel: 100,
      camera_center: vector::Vector::new(0.0, 0.0, 0.0),
      pixel00_loc: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_u: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_v: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_samples_scale: 0.0,
      max_depth: 50,
    };

    // Initialize computed fields
    camera.initialize();

    camera
  }
}

impl Camera {
  // Constructor that initializes the camera with image width and aspect ratio.
  pub fn new(aspect_ratio: f64, image_width: usize, samples_per_pixel: u32) -> Self {
    let mut camera = Camera {
      aspect_ratio,
      image_width,
      image_height: 0,
      samples_per_pixel,
      camera_center: vector::Vector::new(0.0, 0.0, 0.0),
      pixel00_loc: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_u: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_delta_v: vector::Vector::new(0.0, 0.0, 0.0),
      pixel_samples_scale: 0.0,
      max_depth: 50,
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

    self.pixel_samples_scale = 1.0 / self.samples_per_pixel as f64;
  }
}

impl Camera {
  pub fn render(&self, world: &dyn hittable::Hittable) {
    let mut count = 1;
    let mut percentage: u8 = 0;
    let max = self.image_width * self.image_height;

    let mut buffer: RgbImage = ImageBuffer::new(self.image_width as u32, self.image_height as u32);
    for (x, y, pixel) in buffer.enumerate_pixels_mut() {
      if ((count as f64 / max as f64) * 100.0) as u8 > percentage {
        println!("percent: {percentage}");
        percentage = ((count as f64 / max as f64) * 100.0) as u8;
      }

      let mut pixel_colour = vector::Vector::new(0.0, 0.0, 0.0);

      for _ in 0..=self.samples_per_pixel {
        let r = self.get_ray(x, y);
        pixel_colour = pixel_colour + Camera::ray_colour(&r, self.max_depth, world);
      }

      let (r, g, b) = colour::get_colour(&(pixel_colour * self.pixel_samples_scale));
      *pixel = Rgb([r, g, b]);

      count += 1;
    }

    buffer.save("img.png").unwrap();

    println!("DONE!");
  }

  pub fn get_ray(&self, i: u32, j: u32) -> ray::Ray {
    let offset = Self::sample_square();
    let pixel_sample = self.pixel00_loc
      + (self.pixel_delta_u * (i as f64 + offset.x))
      + (self.pixel_delta_v * (j as f64 + offset.y));

    let ray_origin = self.camera_center;
    let ray_direction = pixel_sample - ray_origin;

    ray::Ray::new(ray_origin, ray_direction)
  }

  pub fn sample_square() -> vector::Vector {
    vector::Vector::new(utility::random_df() - 0.5, utility::random_df() - 0.5, 0.0)
  }

  pub fn ray_colour(r: &ray::Ray, depth: u32, world: &dyn hittable::Hittable) -> vector::Vector {
    if depth <= 0 {
      return vector::Vector::new(0.0, 0.0, 0.0);
    }
    let mut hit_rec = hittable::HitRecord::new(
      vector::Vector::new(0.0, 0.0, 0.0),
      vector::Vector::new(0.0, 0.0, 0.0),
      0.0,
      false,
    );
    if world.hit(
      r,
      interval::Interval::new(0.001, utility::INFINITY),
      &mut hit_rec,
    ) {
      let direction = hit_rec.normal.random_on_hemisphere();
      return Camera::ray_colour(&ray::Ray::new(hit_rec.point, direction), depth - 1, world) * 0.5;
    } else {
      let unit_dir = vector::Vector::unit_vector(&r.dir);
      let a = (unit_dir.y + 1.0) * 0.5;

      vector::Vector::new(1.0, 1.0, 1.0) * (1.0 - a) + vector::Vector::new(0.5, 0.7, 1.0) * a
    }
  }
}
