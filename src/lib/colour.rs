use crate::lib::interval;
use crate::lib::vector::Vector;

pub fn get_colour(v: &Vector) -> (u8, u8, u8) {
  let intensity = interval::Interval::new(0.0, 0.999);
  (
    (256.0 * intensity.clamp(v.x)) as u8,
    (256.0 * intensity.clamp(v.y)) as u8,
    (256.0 * intensity.clamp(v.z)) as u8,
  )
}
