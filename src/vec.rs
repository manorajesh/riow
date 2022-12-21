use std::ops::{Index, AddAssign, MulAssign, Neg, DivAssign, Add, Sub, Mul, Div};

#[derive(Debug, Clone)]
pub struct vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Index<i32> for vec3 {
    type Output = f64;

    fn index(&self, index: i32) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("buffer overflow: {} > 2", index)
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
pub type point3 = vec3; // 3D point
pub type color = vec3;  // RGB color

// vec3 Utility Functions

impl std::fmt::Display for vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Add for vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec3::from(
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2]
        )
    }
}

impl Sub for vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3::from(
            self[0] - rhs[0],
            self[1] - rhs[1],
            self[2] - rhs[2]
        )
    }
}

impl Mul for vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3::from(
            self[0] * rhs[0],
            self[1] * rhs[1],
            self[2] * rhs[2]
        )
    }
}

impl Mul<f64> for vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3::from(
            self[0] * rhs,
            self[1] * rhs,
            self[2] * rhs
        )
    }
}

// communitive
impl Mul<vec3> for f64 {
    type Output = vec3;

    fn mul(self, rhs: vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for vec3 {
    type Output = vec3;

    fn div(self, rhs: f64) -> Self::Output {
        (1./rhs) * self
    }
}

impl vec3 {
    pub fn dot(self, other: vec3) -> f64 {
            self[0] * other[0] +
            self[1] * other[1] +
            self[2] * other[2]
    }

    pub fn cross(self, other: vec3) -> vec3 {
        vec3::from(
            self[1] * other[2] - self[2] * other[1],
            self[2] * other[0] - self[0] * other[2],
            self[0] * other[1] - self[1] * other[0]
        )
    }

    pub fn unit_vector(self) -> vec3 {
        self.clone() / self.length()
    }
}