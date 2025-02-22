mod lib;
use lib::{camera, hittable_list, sphere, vector};
use std::rc;

fn main() {
  // make hit world
  let mut world = hittable_list::HittableList::new_empty();

  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, 0.0, -1.0),
    0.5,
  )));
  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, -100.5, -1.0),
    100.0,
  )));

  let c = camera::Camera::default();

  c.render(&world);
}
