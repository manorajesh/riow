use crate::{libvec::{point3, vec3}, libray::ray};

pub struct camera {
    aspect_ratio: f64,
    viewport_height: f64,
    viewport_width: f64,
    focal_length: f64,
    origin: point3,
    horizontal: vec3,
    vertical: vec3,
    lower_left_corner: point3,
}

impl camera {
    pub fn default() -> camera {
        let aspect_ratio = 16./9.;
        let viewport_height = 2.;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = point3::from(0., 0., 0.);
        let horizontal = vec3::from(viewport_width, 0., 0.);
        let vertical = vec3::from(0., viewport_height, 0.);
        let lower_left_corner = origin - horizontal/2. - vertical/2. - vec3::from(0., 0., focal_length);

        camera {
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> ray {
        ray::from(self.origin, self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}