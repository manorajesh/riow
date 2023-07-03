use rand::Rng;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy)]
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
            _ => panic!("buffer overflow: {} > 2", index),
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
        *self *= 1.0 / rhs
    }
}

impl vec3 {
    pub fn new() -> vec3 {
        vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }

    pub fn from(x1: f64, y1: f64, z1: f64) -> vec3 {
        vec3 {
            x: x1,
            y: y1,
            z: z1,
        }
    }

    #[inline(always)]
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn random() -> vec3 {
        let mut rng = rand::thread_rng();
        vec3::from(rng.gen(), rng.gen(), rng.gen())
    }

    pub fn random_range(min: f64, max: f64) -> vec3 {
        let mut rng = rand::thread_rng();
        vec3::from(
            rng.gen_range(min..max),
            rng.gen_range(min..max),
            rng.gen_range(min..max),
        )
    }

    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        let s: f64 = 1e-8;
        (self.x.abs() < s) && (self.y.abs() < s) && (self.z.abs() < s)
    }
}

// Type aliases for vec3
pub type point3 = vec3; // 3D point
pub type color = vec3; // RGB color

// vec3 Utility Functions

impl std::fmt::Display for vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self[0], self[1], self[2])
    }
}

impl Add for vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        vec3::from(self[0] + rhs[0], self[1] + rhs[1], self[2] + rhs[2])
    }
}

impl Sub for vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        vec3::from(self[0] - rhs[0], self[1] - rhs[1], self[2] - rhs[2])
    }
}

impl Mul for vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        vec3::from(self[0] * rhs[0], self[1] * rhs[1], self[2] * rhs[2])
    }
}

impl Mul<f64> for vec3 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        vec3::from(self[0] * rhs, self[1] * rhs, self[2] * rhs)
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
        (1. / rhs) * self
    }
}

#[inline]
pub fn dot(vector: vec3, other: vec3) -> f64 {
    vector.x * other.x + vector.y * other.y + vector.z * other.z
}

pub fn cross(vector: vec3, other: vec3) -> vec3 {
    vec3::from(
        vector[1] * other[2] - vector[2] * other[1],
        vector[2] * other[0] - vector[0] * other[2],
        vector[0] * other[1] - vector[1] * other[0],
    )
}

pub fn unit_vector(vector: vec3) -> vec3 {
    vector / vector.length()
}

pub fn random_in_unit_sphere() -> vec3 {
    loop {
        let p = vec3::random_range(-1., 1.);
        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> vec3 {
    unit_vector(random_in_unit_sphere())
}

pub fn random_in_hemisphere(normal: vec3) -> vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn reflect(v: &vec3, n: &vec3) -> vec3 {
    *v - 2. * dot(*v, *n) * *n
}

pub fn refract(uv: &vec3, n: &vec3, etai_over_etat: f64) -> vec3 {
    let cos_theta = min(dot(-*uv, *n), 1.);
    let r_out_perp = etai_over_etat * (*uv + cos_theta * *n);
    let r_out_parallel = -((1. - r_out_perp.length_squared()).abs()).sqrt() * *n;
    r_out_perp + r_out_parallel
}

pub fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

pub fn random_in_unit_disk() -> vec3 {
    loop {
        let p = vec3::from(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
            0.,
        );

        if p.length_squared() >= 1. {
            continue;
        }
        return p;
    }
}
