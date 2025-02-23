use crate::lib::utility;
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

  pub fn random_unit_vector() -> Vector {
    loop {
      let p = Vector::random_df();
      let lensq = p.length_squared();
      if 1e-160 < lensq && lensq <= 1.0 {
        return p / lensq.sqrt();
      }
    }
  }

  pub fn random_on_hemisphere(&self) -> Vector {
    let on_unit_sphere = Vector::random_unit_vector();
    if (on_unit_sphere.dot(self) > 0.0) {
      return on_unit_sphere;
    } else {
      return on_unit_sphere * -1.0;
    }
  }

  pub fn random_df() -> Vector {
    Vector {
      x: utility::random_df(),
      y: utility::random_df(),
      z: utility::random_df(),
    }
  }

  pub fn random(min: f64, max: f64) -> Vector {
    Vector {
      x: utility::random(min, max),
      y: utility::random(min, max),
      z: utility::random(min, max),
    }
  }

  pub fn near_zero(&self) -> bool {
    let s = 1e-8;
    (self.x < s) && (self.y < s) && (self.z < s)
  }

  pub fn reflect(&self, n: Vector) -> Vector {
    *self - (n * (self.dot(&n) * 2.0))
  }

  pub fn refract(&self, n: Vector, etai_over_etat: f64) -> Vector {
    let cos_theta = (*self * -1.0).dot(&n).min(1.0);
    let r_out_perp = (*self + (n * cos_theta)) * etai_over_etat;
    let r_out_parralel = n * -((1.0 - r_out_perp.length_squared()).abs());
    return r_out_perp + r_out_parralel;
  }

  pub fn cross(&self, v: &Vector) -> Vector {
    Vector::new(
      (self.y * v.z) - (self.z * v.y),
      (self.z * v.x) - (self.x * v.z),
      (self.x * v.y) - (self.y * v.x),
    )
  }

  pub fn random_in_unit_disk() -> Vector {
    loop {
      let p = Vector::new(utility::random(-1.0, 1.0), utility::random(-1.0, 1.0), 0.0);
      if p.length_squared() < 1.0 {
        return p;
      }
    }
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

impl ops::Mul<Vector> for Vector {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self {
      x: self.x * rhs.x,
      y: self.y * rhs.y,
      z: self.z * rhs.z,
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
