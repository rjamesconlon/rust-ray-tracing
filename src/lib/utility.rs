use rand::prelude::*;

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = std::f64::INFINITY;

pub fn deg_to_rad(d: f64) -> f64 {
  d.to_radians()
}

pub fn random_df() -> f64 {
  let mut r = rand::rng();
  r.random_range(0.0..1.0)
}

pub fn random(min: f64, max: f64) -> f64 {
  let mut r = rand::rng();
  r.random_range(min..max)
}
