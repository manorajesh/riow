use crate::{libvec::{color, random_unit_vector, reflect, unit_vector, dot, refract, min}, libray::ray, libhittable::hit_record};

pub enum material {
    Lambertian(lambertian),
    Metal(metal),
    Dielectric(dielectric),
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

pub struct dielectric {
    ior: f64, // Index of Refraction
}

impl dielectric {
    pub fn from(ior: f64) -> dielectric {
        dielectric {
            ior
        }
    }

    pub fn scatter(&self, r_in: &ray, rec: &hit_record, attenuation: &mut color, scattered: &mut ray) -> bool {
        *attenuation = color::from(1., 1., 1.);
        let refraction_ratio = if rec.front_face { 1./self.ior } else { self.ior };

        let unit_direction = unit_vector(r_in.direction);
        let cos_theta = min(dot(-unit_direction, rec.normal), 1.);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction;

        if cannot_refract || reflectance(cos_theta, refraction_ratio) > rand::random::<f64>(){
            direction = reflect(&unit_direction, &rec.normal);
        } else {
            direction = refract(&unit_direction, &rec.normal, refraction_ratio);
        }

        *scattered = ray::from(rec.p, direction);
        true
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance
    let mut r0 = (1.-ref_idx) / (1.+ref_idx);
    r0 = r0*r0;
    r0 + (1.-r0)*((1.-cosine).powi(5))
}