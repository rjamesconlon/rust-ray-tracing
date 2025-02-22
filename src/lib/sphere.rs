use crate::lib::hittable;
use crate::lib::ray::Ray;
use crate::lib::vector::Vector;

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
  fn hit(&self, r: &Ray, ray_min: f64, ray_max: f64, hit: &mut hittable::HitRecord) -> bool {
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
    if root <= ray_min || ray_max <= root {
      root = (h + sqrtd) / a;
      if root <= ray_min || ray_max <= root {
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
