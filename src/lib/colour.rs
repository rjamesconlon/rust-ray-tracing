use crate::lib::vector::Vector;

pub fn get_colour(v: &Vector) -> (u8, u8, u8) {
  (
    (v.x * 255.999) as u8,
    (v.y * 255.999) as u8,
    (v.z * 255.999) as u8,
  )
}
