use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
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

  pub fn length_squared(&self) -> f64 {
    (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
  }

  pub fn length(&self) -> f64 {
    self.length_squared().sqrt()
  }

  pub fn dot(&self, v: &Vec) -> f64 {
    (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
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

impl ops::Sub for Vec {
  type Output = Vec;

  fn sub(self, v: Vec) -> Vec {
    Vec {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
    }
  }
}

impl ops::Mul<f64> for Vec {
  type Output = Vec;

  fn mul(self, i: f64) -> Vec {
    Vec {
      x: self.x * i,
      y: self.y * i,
      z: self.z * i,
    }
  }
}

impl ops::Div<f64> for Vec {
  type Output = Vec;

  fn div(self, i: f64) -> Vec {
    Vec {
      x: self.x / i,
      y: self.y / i,
      z: self.z / i,
    }
  }
}
