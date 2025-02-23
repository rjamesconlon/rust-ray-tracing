mod lib;
use lib::{camera, hittable_list, material, sphere, vector};
use std::rc;

fn main() {
  // make hit world
  let mut world = hittable_list::HittableList::new_empty();

  let material_ground = rc::Rc::new(material::Lambertian {
    albedo: vector::Vector::new(0.8, 0.8, 0.0),
  });

  let material_center = rc::Rc::new(material::Lambertian {
    albedo: vector::Vector::new(0.1, 0.2, 0.5),
  });

  let material_left = rc::Rc::new(material::Metal {
    albedo: vector::Vector::new(0.8, 0.8, 0.8),
  });

  let material_right = rc::Rc::new(material::Metal {
    albedo: vector::Vector::new(0.8, 0.6, 0.2),
  });

  // ground
  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, -100.5, -1.0),
    100.0,
    material_ground.clone(),
  )));

  // center sphere
  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, 0.0, -1.2),
    0.5,
    material_center.clone(),
  )));

  // left sphere
  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(-1.0, 0.0, -1.0),
    0.5,
    material_left.clone(),
  )));

  // right sphere
  world.add(rc::Rc::new(sphere::Sphere::new(
    vector::Vector::new(1.0, 0.0, -1.0),
    0.5,
    material_right.clone(),
  )));

  let c = camera::Camera::default();

  c.render(&world);
}
