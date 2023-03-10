use std::sync::Arc;

use crate::{libvec::*, libray::ray, libhittable::hit_record, libmaterial::material};

pub struct sphere {
    pub center: point3,
    pub radius: f64,
    pub m: Arc<material>,
}

impl sphere {
    pub fn from(center: point3, radius: f64, m: &Arc<material>) -> sphere {
        sphere {
            center,
            radius,
            m: m.clone()
        }
    }

    #[inline(always)]
    pub fn hit(&self, r: ray, t_min: f64, t_max: f64, rec: &mut hit_record) -> bool {
        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(oc, r.direction);
        let c = oc.length_squared() - self.radius*self.radius;

        let discriminant = half_b*half_b - a*c;
        if discriminant < 0. {
            false
        } else {
            let sqrtd = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range
            let mut root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    false
                } else {
                    // not clean code (repeating)
                    rec.t = root;
                    rec.p = r.at(rec.t);
                    let outward_normal = (rec.p - self.center) / self.radius;
                    rec.set_face_normal(r, outward_normal);
                    rec.mat = self.m.clone();
    
                    true 
                }
            } else {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);
                rec.mat = self.m.clone();

                true
            }
        }
    }
}