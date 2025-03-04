use crate::lib::{hittable, interval, material, ray, vector};
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
    ray_t: interval::Interval,
    hit_rec: &mut hittable::HitRecord,
  ) -> bool {
    let mut hit_anything = false;
    let mut closest_so_far = ray_t.max;

    for object in self.objects.iter() {
      let mut temp_rec = hittable::HitRecord::new_empty(); // No material yet
      if object.hit(
        r,
        interval::Interval::new(ray_t.min, closest_so_far),
        &mut temp_rec,
      ) {
        hit_anything = true;
        closest_so_far = temp_rec.t;
        *hit_rec = temp_rec;
      }
    }
    hit_anything
  }
}
