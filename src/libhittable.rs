

use std::rc::Rc;

use crate::{libvec::{point3, vec3, dot, color}, libray::ray, libsphere::sphere, libmaterial::{material, lambertian}};

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

#[derive(Clone)]
pub struct hit_record {
    pub p: point3,
    pub normal: vec3,
    pub t: f64,
    pub mat: Rc<material>,
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
            mat: Rc::new(material::Lambertian(lambertian::from(color::new()))),
            front_face: false
        }
    }
}

pub trait scatter {
    fn scatter(&self, r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool;
}

impl scatter for Rc<material> {
    fn scatter(&self, r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        match self.as_ref() {
            material::Lambertian(l) => l.scatter(r_in, rec, attenuation, scattered),
            material::Metal(m) => m.scatter(r_in, rec, attenuation, scattered),
            material::Dielectric(d) => d.scatter(r_in, rec, attenuation, scattered),
        }
    }
}

#[macro_export]
macro_rules! sphere {
    ($x:expr, $y:expr, $z:expr, $radius:expr, $material:expr) => {
        hittable::Sphere(sphere::from(point3::from($x, $y, $z), $radius, $material))
    };

    ($center:expr, $radius:expr, $material:expr) => {
        hittable::Sphere(sphere::from($center, $radius, $material))
    };
}