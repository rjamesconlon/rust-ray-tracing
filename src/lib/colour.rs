use crate::lib::interval;
use crate::lib::vector::Vector;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
  if linear_component > 0.0 {
    return linear_component.sqrt();
  }
  0.0
}

pub fn get_colour(v: &Vector) -> (u8, u8, u8) {
  let intensity = interval::Interval::new(0.0, 0.999);
  (
    (256.0 * intensity.clamp(linear_to_gamma(v.x))) as u8,
    (256.0 * intensity.clamp(linear_to_gamma(v.y))) as u8,
    (256.0 * intensity.clamp(linear_to_gamma(v.z))) as u8,
  )
}
