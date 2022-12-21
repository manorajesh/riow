use crate::{libvec::{point3, vec3, dot}, libray::ray, libsphere::sphere};

pub enum hittable {
    Sphere(sphere),
}

impl hittable {
    pub fn hit(&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        match self {
            hittable::Sphere(s) => s.hit(r, t_min, t_max, rec),
        }
    }
}

#[derive(Clone, Copy)]
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

    pub fn new() -> hit_record {
        hit_record {
            p: point3::new(),
            normal: vec3::new(),
            t: 0.,
            front_face: false
        }
    }
}