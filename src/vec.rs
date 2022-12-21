use std::ops::{Index, AddAssign, MulAssign, Neg, DivAssign};

#[derive(Debug)]
pub struct vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Index<i32> for vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("buffer overflow")
        }
    }
}

impl Neg for vec3 {
    type Output = vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for vec3 {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl MulAssign<f64> for vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        *self = Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl DivAssign<f64> for vec3 {
    fn div_assign(&mut self, rhs: f64) {
        *self *= 1.0/rhs
    }
}
impl vec3 {
    pub fn new() -> vec3 {
        vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn from(x1: f64, y1: f64, z1: f64) -> vec3 {
        vec3 { x: x1, y: y1, z: z1 }
    }

    pub fn length_squared(self) -> f64 {
        self.x*self.x + self.y*self.y + self.z*self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
}

// Type aliases for vec3
type point3 = vec3; // 3D point
type color = vec3;  // RGB color