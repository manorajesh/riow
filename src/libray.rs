use crate::libvec::{point3, vec3};

#[derive(Clone, Copy)]
pub struct ray {
    pub origin: point3,
    pub direction: vec3,
}

impl ray {
    pub fn new() -> ray {
        ray {
            origin: point3::new(),
            direction: vec3::new(),
        }
    }

    pub fn from(orig: point3, dir: vec3) -> ray {
        ray {
            origin: orig,
            direction: dir,
        }
    }

    pub fn at(self, t: f64) -> vec3 {
        self.origin + t * self.direction
    }
}
