use crate::lib::{hittable, ray, vector};
use std::rc;

pub struct HittableList {
  pub objects: Vec<rc::Rc<dyn hittable::Hittable>>,
}

impl HittableList {
  pub fn new_empty() -> HittableList {
    HittableList {
      objects: Vec::new(),
    }
  }

  pub fn new(&mut self, object: rc::Rc<dyn hittable::Hittable>) -> HittableList {
    let mut h = HittableList {
      objects: Vec::new(),
    };

    h.objects.push(object);

    return h;
  }

  pub fn add(&mut self, object: rc::Rc<dyn hittable::Hittable>) {
    self.objects.push(object);
  }

  pub fn clear(&mut self) {
    self.objects.clear();
  }
}

impl hittable::Hittable for HittableList {
  fn hit(
    &self,
    r: &ray::Ray,
    ray_min: f64,
    ray_max: f64,
    hit_rec: &mut hittable::HitRecord,
  ) -> bool {
    let mut hit_anything = false;
    let mut closest_so_far = ray_max;

    for object in self.objects.iter() {
      let mut temp_rec = hittable::HitRecord::new(
        vector::Vector::new(0.0, 0.0, 0.0),
        vector::Vector::new(0.0, 0.0, 0.0),
        0.0,
        false,
      );
      if object.hit(r, ray_min, closest_so_far, &mut temp_rec) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *hit_rec = temp_rec;
      }
    }
    hit_anything
  }
}
