use crate::ray;
use crate::vector;

pub struct HitRecord {
  pub point: vector::Vector,
  pub normal: vector::Vector,
  pub t: f64,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(point: vector::Vector, normal: vector::Vector, t: f64, front_face: bool) -> HitRecord {
    HitRecord {
      point,
      normal,
      t,
      front_face,
    }
  }

  pub fn set_face_normal(&mut self, r: &ray::Ray, v: vector::Vector) {
    // sets hit record normal vector
    // v has assumed unit length (normal)

    self.front_face = r.dir.dot(&v) < 0.0;
    self.normal = if self.front_face { v } else { v * -1.0 }
  }
}

pub trait Hittable {
  fn hit(&self, r: &ray::Ray, ray_min: f64, ray_max: f64, hit_rec: &mut HitRecord) -> bool;
}
