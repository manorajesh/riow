#[allow(non_camel_case_types)]
mod libvec;
mod libcolor;
mod libray;
mod libhittable;
mod libsphere;

use libvec::*;
use libcolor::write_color;
use libray::*;
use std::io::{stderr, Write};

fn hit_sphere(center: point3, radius: f64, r: ray) -> f64 {
    let oc = r.origin - center;

    let a = r.direction.length_squared();
    let half_b = dot(oc, r.direction);
    let c = oc.length_squared() - radius*radius;
    let discriminant = half_b*half_b - a*c;

    // checking for roots or intersects on circle
    if discriminant < 0. {
        -1.
    } else {
        (-half_b - discriminant.sqrt()) / (2.*a)
    }
}

fn ray_color(r: ray) -> color {
    let t = hit_sphere(point3::from(0., 0., -1.), 0.5, r);
    if t > 0. {
        let N = unit_vector(r.at(t) - vec3::from(0., 0., -1.));
        0.5*color::from(N.x+1., N.y+1., N.z+1.)
    } else {
        let unit_direction = unit_vector(r.direction);
        let t = 0.5 * (unit_direction.y + 1.);
        (1.-t)*color::from(1., 1., 1.) + t*color::from(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    let aspect_ratio = 16./9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64/ aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = point3::from(0., 0., 0.);
    let horizontal = vec3::from(viewport_width, 0., 0.);
    let vertical = vec3::from(0., viewport_height, 0.);
    let lower_left_corner = origin - horizontal/2. - vertical/2. - vec3::from(0., 0., focal_length);

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            let u = i as f64/(image_width-1) as f64;
            let v = j as f64/(image_height-1) as f64;
            let r = ray::from(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            write_color(pixel_color);
        }
    }
    eprint!("\nDone!\n");
}