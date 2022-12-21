#[allow(non_camel_case_types)]
mod vec;
mod libcolor;

use vec::color;
use libcolor::write_color;
use std::io::{stderr, Write};

fn main() {
    // Image
    let image_width: i32 = 256;
    let image_height: i32 = 256;

    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..image_width {
            let pixel_color = color::from(i as f64/(image_width-1) as f64, j as f64/(image_height-1) as f64, 0.25);
            write_color(pixel_color);
        }
    }
    eprint!("\nDone!\n");
}