use crate::lib::{colour, hittable, interval, utility, vector};

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

  pub fn ray_colour(&self, world: &dyn hittable::Hittable) -> vector::Vector {
    let mut hit_rec = hittable::HitRecord::new(
      vector::Vector::new(0.0, 0.0, 0.0),
      vector::Vector::new(0.0, 0.0, 0.0),
      0.0,
      false,
    );
    if world.hit(
      self,
      interval::Interval::new(0.0, utility::INFINITY),
      &mut hit_rec,
    ) {
      return (hit_rec.normal + vector::Vector::new(1.0, 1.0, 1.0)) * 0.5;
    } else {
      let unit_dir = vector::Vector::unit_vector(&self.dir);
      let a = (unit_dir.y + 1.0) * 0.5;

      vector::Vector::new(1.0, 1.0, 1.0) * (1.0 - a) + vector::Vector::new(0.5, 0.7, 1.0) * a
    }
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
