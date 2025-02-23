mod lib;
use lib::{camera, hittable_list, material, sphere, utility, vector};
use std::rc::Rc;

fn main() {
  let mut world = hittable_list::HittableList::new_empty();

  let ground_material = Rc::new(material::Lambertian {
    albedo: vector::Vector::new(0.5, 0.5, 0.5),
  });
  world.add(Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, -1000.0, 0.0),
    1000.0,
    ground_material.clone(),
  )));

  for a in -11..11 {
    for b in -11..11 {
      let choose_mat = utility::random_df();
      let center = vector::Vector::new(
        a as f64 + 0.9 * utility::random_df(),
        0.2,
        b as f64 + 0.9 * utility::random_df(),
      );

      if (center - vector::Vector::new(4.0, 0.2, 0.0)).length() > 0.9 {
        let sphere_material: Rc<dyn material::Material> = if choose_mat < 0.8 {
          // Diffuse
          Rc::new(material::Lambertian {
            albedo: vector::Vector::random_df() * vector::Vector::random_df(),
          })
        } else if choose_mat < 0.95 {
          // Metal
          Rc::new(material::Metal {
            albedo: vector::Vector::random(0.5, 1.0),
            fuzz: utility::random(0.0, 0.5),
          })
        } else {
          // Glass
          Rc::new(material::Dialetric {
            refraction_index: 1.5,
          })
        };
        world.add(Rc::new(sphere::Sphere::new(center, 0.2, sphere_material)));
      }
    }
  }

  let material1 = Rc::new(material::Dialetric {
    refraction_index: 1.5,
  });
  world.add(Rc::new(sphere::Sphere::new(
    vector::Vector::new(0.0, 1.0, 0.0),
    1.0,
    material1.clone(),
  )));

  let material2 = Rc::new(material::Lambertian {
    albedo: vector::Vector::new(0.4, 0.2, 0.1),
  });
  world.add(Rc::new(sphere::Sphere::new(
    vector::Vector::new(-4.0, 1.0, 0.0),
    1.0,
    material2.clone(),
  )));

  let material3 = Rc::new(material::Metal {
    albedo: vector::Vector::new(0.7, 0.6, 0.5),
    fuzz: 0.0,
  });
  world.add(Rc::new(sphere::Sphere::new(
    vector::Vector::new(4.0, 1.0, 0.0),
    1.0,
    material3.clone(),
  )));

  let aspect_ratio = 16.0 / 9.0;
  let image_width = 400;
  let samples_per_pixel = 400;
  let max_depth = 50;

  let lookfrom = vector::Vector::new(13.0, 2.0, 3.0);
  let lookat = vector::Vector::new(0.0, 0.0, 0.0);
  let vup = vector::Vector::new(0.0, 1.0, 0.0);
  let vfov = 20.0;

  let defocus_angle = 0.6;
  let focus_dist = 10.0;

  let cam = camera::Camera::new(
    aspect_ratio,
    image_width,
    samples_per_pixel,
    max_depth,
    lookfrom,
    lookat,
    vup,
    vfov,
    defocus_angle,
    focus_dist,
  );

  cam.render(&world);
}
