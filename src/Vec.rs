use std::ops;

pub struct Vec {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vec {
  pub fn new(x: f64, y: f64, z: f64) -> Vec {
    Vec { x, y, z }
  }

  pub fn get_vector(&self) -> (f64, f64, f64) {
    (self.x, self.y, self.z)
  }
}

impl ops::Add for Vec {
  type Output = Vec;

  fn add(self, v: Vec) -> Vec {
    Vec {
      x: self.x + v.x,
      y: self.y + v.y,
      z: self.z + v.z,
    }
  }
}
