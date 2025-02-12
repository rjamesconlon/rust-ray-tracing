use crate::colour::get_colour;
use crate::vector::Vector;

pub struct Ray {
  pub orig: Vector,
  pub dir: Vector,
}

impl Ray {
  pub fn new(orig: Vector, dir: Vector) -> Ray {
    Ray { orig, dir }
  }

  pub fn at(&self, t: f64) -> Vector {
    self.orig + (self.dir * t)
  }

  pub fn ray_colour(&self) -> (u8, u8, u8) {
    let t = self.hit_sphere(Vector::new(0.0, 0.0, -1.0), 0.5);
    if t > 0.0 {
      let n = (self.at(t) - Vector::new(0.0, 0.0, -1.0)).unit_vector();
      return get_colour(&{ Vector::new(n.x + 1.0, n.y + 1.0, n.z + 1.0) * 0.5 });
    } else {
      let unit_dir = Vector::unit_vector(&self.dir);
      let a = (unit_dir.y + 1.0) * 0.5;
      get_colour(&{ Vector::new(1.0, 1.0, 1.0) * (1.0 - a) + Vector::new(0.5, 0.7, 1.0) * a })
    }
  }

  pub fn hit_sphere(&self, center: Vector, radius: f64) -> f64 {
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
