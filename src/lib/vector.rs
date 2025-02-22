use std::ops;

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vector {
  pub x: f64,
  pub y: f64,
  pub z: f64,
}

impl Vector {
  pub fn new(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z }
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

  pub fn dot(&self, v: &Vector) -> f64 {
    (self.x * v.x) + (self.y * v.y) + (self.z * v.z)
  }

  pub fn unit_vector(&self) -> Vector {
    *self / self.length()
  }
}

impl ops::Add for Vector {
  type Output = Vector;

  fn add(self, v: Vector) -> Vector {
    Vector {
      x: self.x + v.x,
      y: self.y + v.y,
      z: self.z + v.z,
    }
  }
}

impl ops::Sub for Vector {
  type Output = Vector;

  fn sub(self, v: Vector) -> Vector {
    Vector {
      x: self.x - v.x,
      y: self.y - v.y,
      z: self.z - v.z,
    }
  }
}

impl ops::Mul<f64> for Vector {
  type Output = Vector;

  fn mul(self, i: f64) -> Vector {
    Vector {
      x: self.x * i,
      y: self.y * i,
      z: self.z * i,
    }
  }
}

impl ops::Div<f64> for Vector {
  type Output = Vector;

  fn div(self, i: f64) -> Vector {
    Vector {
      x: self.x / i,
      y: self.y / i,
      z: self.z / i,
    }
  }
}
