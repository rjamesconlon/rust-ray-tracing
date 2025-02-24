use crate::lib::vector;

pub struct Ray {
  pub orig: vector::Vector,
  pub dir: vector::Vector,
}

impl Ray {
  pub fn new(orig: vector::Vector, dir: vector::Vector) -> Ray {
    Ray { orig, dir }
  }

  pub fn at(&self, t: f64) -> vector::Vector {
    self.orig + (self.dir * t)
  }

  pub fn hit_sphere(&self, center: vector::Vector, radius: f64) -> f64 {
    let oc = center - self.orig;
    let a = self.dir.length_squared();
    let h = self.dir.dot(&oc);
    let c = oc.length_squared() - (radius * radius);
    let discriminant = (h * h) - (a * c);
    if discriminant < 0.0 {
      return -1.0;
    } else {
      return (h - discriminant.sqrt()) / a;
    }
  }
}
