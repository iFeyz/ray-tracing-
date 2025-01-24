use crate::ray::vec3::Vec3;

pub fn write_color(pixel_color: Vec3) {
    let r = (pixel_color.x() * 255.999) as i32;
    let g = (pixel_color.y() * 255.999) as i32;
    let b = (pixel_color.z() * 255.999) as i32;
    println!("{} {} {}", r, g, b);
}