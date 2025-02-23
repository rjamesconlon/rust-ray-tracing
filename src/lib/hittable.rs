use crate::lib::{interval, material, ray, vector};
use std::rc;

pub struct HitRecord {
  pub point: vector::Vector,
  pub normal: vector::Vector,
  pub t: f64,
  pub mat: rc::Rc<dyn material::Material>,
  pub front_face: bool,
}

impl HitRecord {
  pub fn new(
    point: vector::Vector,
    normal: vector::Vector,
    mat: rc::Rc<dyn material::Material>,
    t: f64,
    front_face: bool,
  ) -> HitRecord {
    HitRecord {
      point,
      normal,
      t,
      mat,
      front_face,
    }
  }
  pub fn new_empty() -> Self {
    Self {
      point: vector::Vector::new(0.0, 0.0, 0.0),
      normal: vector::Vector::new(0.0, 0.0, 0.0),
      mat: rc::Rc::new(material::Lambertian {
        albedo: vector::Vector::new(0.0, 0.0, 0.0),
      }), // Placeholder, will be overwritten
      t: 0.0,
      front_face: false,
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
  fn hit(&self, r: &ray::Ray, ray_t: interval::Interval, hit_rec: &mut HitRecord) -> bool;
}
