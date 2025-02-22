use crate::lib::ray::Ray;
use crate::lib::vector::Vector;
use crate::lib::{hittable, interval};

pub struct Sphere {
  pub center: Vector,
  pub radius: f64,
}

impl Sphere {
  pub fn new(center: Vector, radius: f64) -> Sphere {
    Sphere {
      center,
      radius: radius.max(0.0),
    }
  }
}

impl hittable::Hittable for Sphere {
  fn hit(&self, r: &Ray, ray_t: interval::Interval, hit: &mut hittable::HitRecord) -> bool {
    let oc = self.center - r.orig;
    let a = r.dir.length_squared();
    let h = r.dir.dot(&oc);
    let c = oc.length_squared() - (self.radius * self.radius);
    let discriminant = (h * h) - (a * c);
    if discriminant < 0.0 {
      return false;
    };

    let sqrtd = discriminant.sqrt();

    let mut root = (h - sqrtd) / a;
    if !ray_t.surrounds(root) {
      root = (h + sqrtd) / a;
      if !ray_t.surrounds(root) {
        return false;
      }
    };

    hit.t = root;
    hit.point = r.at(hit.t);
    let outward_normal: Vector = (hit.point - self.center) / self.radius;
    hit.set_face_normal(r, outward_normal);

    return true;
  }
}
