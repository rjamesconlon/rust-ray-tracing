use crate::lib::{hittable, ray, utility, vector};

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
    _r_in: &ray::Ray,
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
  pub fuzz: f64,
}

impl Material for Metal {
  fn scatter(
    &self,
    r_in: &ray::Ray,
    rec: &hittable::HitRecord,
    attenuation: &mut vector::Vector,
    scattered: &mut ray::Ray,
  ) -> bool {
    let reflected =
      r_in.dir.reflect(rec.normal) + (vector::Vector::random_unit_vector() * self.fuzz);
    *scattered = ray::Ray::new(rec.point, reflected);
    *attenuation = self.albedo;
    return scattered.dir.dot(&rec.normal) > 0.0;
  }
}

pub struct Dialetric {
  pub refraction_index: f64,
}

impl Material for Dialetric {
  fn scatter(
    &self,
    r_in: &ray::Ray,
    rec: &hittable::HitRecord,
    attenuation: &mut vector::Vector,
    scattered: &mut ray::Ray,
  ) -> bool {
    *attenuation = vector::Vector::new(1.0, 1.0, 1.0);
    let ri = if rec.front_face {
      1.0 / self.refraction_index
    } else {
      self.refraction_index
    };

    let unit_direction = r_in.dir.unit_vector();
    let cos_theta = (unit_direction * -1.0).dot(&rec.normal).min(1.0);
    let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

    let cannot_refract = ri * sin_theta > 1.0;
    let direction =
      if cannot_refract || Dialetric::reflectance(cos_theta, ri) > utility::random_df() {
        unit_direction.reflect(rec.normal)
      } else {
        unit_direction.refract(rec.normal, ri)
      };

    *scattered = ray::Ray::new(rec.point, direction);
    return true;
  }
}

impl Dialetric {
  pub fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 = r0 * r0;
    r0 + ((1.0 - r0) * ((1.0 - cosine).powi(5)))
  }
}
