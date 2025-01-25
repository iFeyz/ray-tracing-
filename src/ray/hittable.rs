use crate::ray::ray::Ray;
use crate::ray::vec3::{Vec3, Point3};
use crate::ray::vec3::dot;
use crate::utils::interval::Interval;

pub trait Hittable {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: Interval,
    pub p: Point3,
    pub normal: Vec3,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            t: Interval::new(0.0, f32::INFINITY),
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, ray: Ray, outward_normal: Vec3) {
        self.front_face = dot(ray.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face { outward_normal } else { -outward_normal };
    }
}