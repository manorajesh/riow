#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

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
use libmaterial::{material, lambertian, metal, dielectric};

use std::io::{stderr, Write};

fn random_scene() -> hittable_list {
    let mut world = hittable_list::new();

    let ground_material = lambertian!(0.5, 0.5, 0.5);
    world.add(sphere!(0., -1000., 0., 1000., &ground_material));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::random::<f64>();
            let center = point3::from(a as f64 + 0.9*rand::random::<f64>(), 0.2, b as f64 + 0.9*rand::random::<f64>());

            if (center - point3::from(4., 0.2, 0.)).length() > 0.9 {
                if choose_mat < 0.4 {
                    // diffuse
                    let albedo = color::random() * color::random();
                    let sphere_material = lambertian!(albedo);
                    world.add(sphere!(center, 0.2, &sphere_material));
                } else if choose_mat < 0.6 {
                    // metal
                    let albedo = color::random_range(0.5, 1.);
                    let roughness = rand::random::<f64>();
                    let sphere_material = metal!(albedo, roughness);
                    world.add(sphere!(center, 0.2, &sphere_material));
                } else {
                    // glass
                    let sphere_material = dielectric!(1.5);
                    world.add(sphere!(center, 0.2, &sphere_material));
                }
            }
        }
    }

    let material1 = dielectric!(1.5);
    world.add(sphere!(point3::from(0., 1., 0.), 1., &material1));

    let material2 = lambertian!(0.4, 0.2, 0.1);
    world.add(sphere!(point3::from(-4., 1., 0.), 1., &material2));

    let material3 = metal!(color::from(0.7, 0.6, 0.5), 0.);
    world.add(sphere!(point3::from(4., 1., 0.), 1., &material3));

    world
}

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

fn main() {
    // Image
    let aspect_ratio = 3./2.;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64/ aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = point3::from(13., 2., 3.);
    let lookat = point3::from(0., 0., 0.);
    let vup = vec3::from(0., 1., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.1;

    let cam = camera::from(lookfrom, lookat, vup, 20., aspect_ratio, aperture, dist_to_focus);

    // Render
    let start_time = std::time::SystemTime::now();

    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            let mut pixel_color = color::new();
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>())/(image_width-1) as f64;
                let v = (j as f64 + rand::random::<f64>())/(image_height-1) as f64;
                let r = cam.get_ray(u, v);
                pixel_color += ray_color(r, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
    }

    eprint!("\nDone! - {} seconds\n", start_time.elapsed().unwrap().as_secs());
}