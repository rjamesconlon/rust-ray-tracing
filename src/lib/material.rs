use crate::lib::{hittable, ray, vector};

pub trait Material {
  fn scatter(
    &self,
    r_in: &ray::Ray,
    rec: &hittable::HitRecord,
    attenuation: &mut vector::Vector,
    scattered: &mut ray::Ray,
  ) -> bool;
}

pub struct Lambertian {
  pub albedo: vector::Vector,
}

impl Material for Lambertian {
  fn scatter(
    &self,
    r_in: &ray::Ray,
    rec: &hittable::HitRecord,
    attenuation: &mut vector::Vector,
    scattered: &mut ray::Ray,
  ) -> bool {
    let mut scatter_direction = rec.normal + vector::Vector::random_unit_vector();

    if scatter_direction.near_zero() {
      scatter_direction = rec.normal;
    }

    *scattered = ray::Ray::new(rec.point, scatter_direction);
    *attenuation = self.albedo;
    return true;
  }
}

pub struct Metal {
  pub albedo: vector::Vector,
}

impl Material for Metal {
  fn scatter(
    &self,
    r_in: &ray::Ray,
    rec: &hittable::HitRecord,
    attenuation: &mut vector::Vector,
    scattered: &mut ray::Ray,
  ) -> bool {
    let reflected = r_in.dir.reflect(rec.normal);
    *scattered = ray::Ray::new(rec.point, reflected);
    *attenuation = self.albedo;
    return true;
  }
}
