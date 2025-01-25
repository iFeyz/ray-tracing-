use crate::ray::ray::Ray;
use crate::ray::vec3::{Vec3, Point3};
use crate::ray::hittable::{Hittable, HitRecord};
use crate::utils::interval::Interval;

pub struct Camera {
    pub camera_center: Point3,
    pub viewport_width: f32,
    pub viewport_height: f32,
    pub focal_length: f32,
    pub aspect_ratio: f32,
    pub image_width: i32,
    pub image_height: i32,
}

impl Camera {
    pub fn new(camera_center: Point3, image_height: f32, aspect_ratio: f32, image_width: i32, focal_length: f32) -> Self {
        let viewport_height = 2.0;
        let viewport_width = viewport_height * aspect_ratio;
        Camera {
            camera_center,
            viewport_width,
            viewport_height,
            focal_length,
            aspect_ratio,
            image_width,
            image_height: image_height as i32,
        }
    }

    pub fn ray_color(ray: Ray, world: &dyn Hittable) -> Vec3 {
        let mut rec = HitRecord::new();
        if world.hit(ray, Interval::new(0.0, f32::INFINITY), &mut rec) {
            return 0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0));
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.9, 0.2, 1.0) * a
    }
}