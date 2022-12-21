use crate::{libvec::{color, random_unit_vector, reflect, unit_vector, dot}, libray::ray, libhittable::hit_record};

pub enum material {
    Lambertian(lambertian),
    Metal(metal),
}

pub struct lambertian {
    albedo: color,
}

impl lambertian {
    pub fn from(albedo: color) -> lambertian {
        lambertian {
            albedo
        }
    }

    pub fn scatter(&self, _r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = ray::from(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

pub struct metal {
    albedo: color,
    roughness: f64, // fuzz
}

impl metal {
    pub fn from(albedo: color, roughness: f64) -> metal {
        metal {
            albedo,
            roughness
        }
    }

    pub fn scatter(&self, r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        let reflected = reflect(&unit_vector(r_in.direction), &rec.normal);
        *scattered = ray::from(rec.p, reflected+self.roughness*random_unit_vector());
        *attenuation = self.albedo;
        dot(scattered.direction, rec.normal) > 0.
    }
}