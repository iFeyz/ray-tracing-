use crate::ray::vec3::Vec3;
use crate::utils::interval::Interval;

pub fn linear_to_gamma(x :f32) -> f32 {
    if x > 0.0 {
        x.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(pixel_color: Vec3, samples_per_pixel: i32) {
    let scale = 1.0 / samples_per_pixel as f32;
    let r = (pixel_color.x() * scale).sqrt();
    let g = (pixel_color.y() * scale).sqrt();
    let b = (pixel_color.z() * scale).sqrt();

    println!("{} {} {}", 
        (256.0 * r.clamp(0.0, 0.999)) as i32,
        (256.0 * g.clamp(0.0, 0.999)) as i32,
        (256.0 * b.clamp(0.0, 0.999)) as i32
    );
}