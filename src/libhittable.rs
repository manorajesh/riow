use crate::{libvec::{point3, vec3, dot}, libray::ray};

pub struct hit_record {
    pub p: point3,
    pub normal: vec3,
    pub t: f64,
    pub front_face: bool
}

impl hit_record {
    pub fn set_face_normal(&mut self, r: ray, outward_normal: vec3) {
        self.front_face = dot(r.direction, outward_normal) < 0.;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}