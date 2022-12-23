use crate::libvec::color;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min { return min; }
    if x > max { return max; }
    x
}

pub fn write_color(pixel_color: color, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x;
    let mut g = pixel_color.y;
    let mut b = pixel_color.z;

    // Divide the color by number of samples and gamma-correct for gamma=2.0
    let scale = 1. / samples_per_pixel as f64;
    r = (scale * r).sqrt();
    g = (scale * g).sqrt();
    b = (scale * b).sqrt();
    
    // Write translated [0, 255] value of each color component
    format!(
        "{} {} {}",
        (256. * clamp(r, 0., 0.999)) as i32,
        (256. * clamp(g, 0., 0.999)) as i32,
        (256. * clamp(b, 0., 0.999)) as i32
    )
}