#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod libvec;
mod libcolor;
mod libray;
mod libhittable;
mod libsphere;
mod libhittable_list;
mod libcamera;
mod libmaterial;

use libhittable::{hittable, hit_record};
use libvec::*;
use libcolor::write_color;
use libray::*;
use libsphere::sphere;
use libhittable_list::hittable_list;
use libcamera::camera;
use libhittable::scatter;

use std::{io::{stderr, Write}, rc::Rc};
use rand::Rng;

use crate::libmaterial::{material, lambertian, metal};

fn ray_color(r: ray, world: &hittable_list, depth: i32) -> color {
    let mut rec = hit_record::new();

    if depth <= 0 {
        return color::new();
    }

    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered = ray::new();
        let mut attenuation = color::new();
        if rec.mat.scatter(&r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(scattered, world, depth-1);
        }
        return color::new();


        // // let target = rec.p + rec.normal + random_in_unit_sphere(); // diffuse scattering
        // // let target = rec.p + rec.normal + random_unit_vector(); // lambertian scattering
        // let target = rec.p + random_in_hemisphere(rec.normal); // hemispherical scattering
        // 0.5 * ray_color(ray::from(rec.p, target-rec.p), world, depth-1)
    } else {
        let unit_direction = unit_vector(r.direction);
        let t = 0.5*(unit_direction.y + 1.);
        (1. - t)*color::from(1., 1., 1.) + t*color::from(0.5, 0.7, 1.)
    }
}

fn random_double() -> f64 {
    rand::thread_rng().gen()
}

fn main() {
    // Image
    let aspect_ratio = 16./9.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64/ aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = hittable_list::new();

    let material_ground = Rc::new(material::Lambertian(lambertian::from(color::from(0.8, 0.8, 0.))));
    let material_center = Rc::new(material::Lambertian(lambertian::from(color::from(0.7, 0.3, 0.3))));
    let material_left = Rc::new(material::Metal(metal::from(color::from(0.8, 0.8, 0.8), 0.3)));
    let material_right = Rc::new(material::Metal(metal::from(color::from(0.8, 0.6, 0.2), 0.)));
    let material_test = Rc::new(material::Metal(metal::from(color::from(0.8, 0.6, 0.2), 0.)));

    world.add(hittable::Sphere(sphere::from(point3::from(0., -100.5, -1.), 100., material_ground)));
    world.add(hittable::Sphere(sphere::from(point3::from(0., 0., -1.), 0.5, material_center)));
    world.add(hittable::Sphere(sphere::from(point3::from(-1., 0., -1.), 0.5, material_left)));
    world.add(hittable::Sphere(sphere::from(point3::from(1., 0., -1.), 0.5, material_right)));
    world.add(hittable::Sphere(sphere::from(point3::from(1., 0., 0.), 0.5, material_test)));

    // Camera
    let cam = camera::default();

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = color::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random_double())/(image_width-1) as f64;
                let v = (j as f64 + random_double())/(image_height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }
    eprint!("\nDone!\n");
}