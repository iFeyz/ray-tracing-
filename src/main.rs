use ray::ray::vec3::{Vec3, Point3};
use ray::ray::color::write_color;
use ray::ray::ray::Ray;
use ray::object::sphere::Sphere;
use ray::object::hittable_list::HittableList;
use ray::utils::camera::Camera;

fn main() {
    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, 1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0.3, -0.2, 1.3), 0.5)));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let focal_length = 1.0;
    let image_height = (image_width as f32 / aspect_ratio) as i16;
    let camera = Camera::new(Point3::new(0.0, 0.0, 0.0), image_height as f32, aspect_ratio, image_width, focal_length);

    // Calc the vectors horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(camera.viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -camera.viewport_height, 0.0);

    // delta from both vectors
    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    // Location of the upper left pixel
    let viewport_upper_left = camera.camera_center 
        - Vec3::new(0.0, 0.0, camera.focal_length) 
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
            let ray_direction = pixel_center - camera.camera_center;
            let ray = Ray::new(camera.camera_center, ray_direction);
            let color = Camera::ray_color(ray, &world);
            write_color(color);
        }
    }
}


