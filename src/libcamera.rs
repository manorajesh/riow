use crate::{libvec::{point3, vec3, unit_vector, cross, random_in_unit_disk}, libray::ray};

pub struct camera {
    origin: point3,
    horizontal: vec3,
    vertical: vec3,
    lower_left_corner: point3,
    lens_radius: f64,
    u: vec3,
    v: vec3,
}

impl camera {
    pub fn from(lookfrom: point3, lookat: point3, vup: vec3, vfox: f64, aspect_ratio: f64, aperture: f64, focus_dist: f64) -> camera {
        let theta = degrees_to_radians(vfox);
        let h = (theta/2.).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = unit_vector(lookfrom - lookat);
        let u = unit_vector(cross(vup, w));
        let v = cross(w, u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2. - vertical/2. - focus_dist * w;

        let lens_radius = aperture / 2.;

        camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            u,
            v,
        }
    }
    
    pub fn get_ray(&self, s: f64, t: f64) -> ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        ray::from(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset
        )
    }
}

fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.
}