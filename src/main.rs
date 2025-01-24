use ray::ray::vec3::{Vec3, Point3};
use ray::ray::color::write_color;
use ray::ray::ray::Ray;
use ray::object::sphere::Sphere;
use ray::ray::hittable::{Hittable, HitRecord};

fn ray_color(ray: Ray) -> Vec3 {
    let sphere = Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5);
    let mut rec = HitRecord::new();
    if sphere.hit(ray, 0.0, f32::INFINITY, &mut rec) {
        let n = (ray.at(rec.t) - Point3::new(0.0, 0.0, -1.0)).unit_vector();
        return Vec3::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
        
    }

    let unit_direction = ray.direction().unit_vector();
    let a = 0.5 * (unit_direction.y() + 1.0);
    Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    // Image height and > 1
    let mut image_height = (image_width as f32 / aspect_ratio) as i16;
    if image_height < 1 {
        image_height = 1;
    }
    
    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f32 / image_height as f32);
    let camera_center = Point3::new(0.0, 0.0, 0.0);
    
    // Calc the vectors horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
    // delta from both vectors
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;
    
    // Location of the upper left pixel
    let viewport_upper_left = camera_center 
        - Vec3::new(0.0, 0.0, focal_length) 
        - viewport_u / 2.0 
        - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) * 0.5;

    // Render
    // PPM header
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255"); // Maximum color value

    // Iterate over each pixel
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center = pixel00_loc 
                + pixel_delta_u * i as f32 
                + pixel_delta_v * j as f32;
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction);
            let color = ray_color(ray);
            write_color(color);
        }
    }
}

