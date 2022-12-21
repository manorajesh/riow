#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod libvec;
mod libcolor;
mod libray;
mod libhittable;
mod libsphere;
mod libhittable_list;

use libhittable::{hittable, hit_record};
use libvec::*;
use libcolor::write_color;
use libray::*;
use libsphere::sphere;
use libhittable_list::hittable_list;
use std::io::{stderr, Write};

fn ray_color(r: ray, world: &hittable_list) -> color {
    let rec = hit_record::new();
    if world.hit(r, 0., f64::INFINITY, rec) {
        0.5 * (rec.normal + color::from(1., 1., 1.))
    } else {
        let unit_direction = unit_vector(r.direction);
        let t = 0.5*(unit_direction.y + 1.);
        (1. - t)*color::from(1., 1., 1.) + t*color::from(0.5, 0.7, 1.)
    }
}

fn main() {
    // Image
    let aspect_ratio = 16./9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64/ aspect_ratio) as i32;

    // World
    let mut world = hittable_list::new();
    world.add(hittable::Sphere(sphere::from(point3::from(0., 0., -1.), 0.5)));
    world.add(hittable::Sphere(sphere::from(point3::from(0., -100.5, -1.), 100.)));

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
            let pixel_color = ray_color(r, &world);
            write_color(pixel_color);
        }
    }
    eprint!("\nDone!\n");
}