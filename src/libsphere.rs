use crate::{libvec::*, libray::ray, libhittable::hit_record};

pub struct sphere {
    pub center: point3,
    pub radius: f64
}

impl sphere {
    pub fn from(center: point3, radius: f64) -> sphere {
        sphere {
            center,
            radius
        }
    }
    pub fn hit(&self, r: ray, t_min: f64, t_max: f64, mut rec: hit_record) -> bool {
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
            let root = (-half_b - sqrtd) / a;
            if root < t_min || t_max < root {
                let root = (-half_b + sqrtd) / a;
                if root < t_min || t_max < root {
                    false
                } else {
                    // not clean code (repeating)
                    rec.t = root;
                    rec.p = r.at(rec.t);
                    let outward_normal = (rec.p - self.center) / self.radius;
                    rec.set_face_normal(r, outward_normal);
    
                    true 
                }
            } else {
                rec.t = root;
                rec.p = r.at(rec.t);
                let outward_normal = (rec.p - self.center) / self.radius;
                rec.set_face_normal(r, outward_normal);

                true
            }
        }
    }
}