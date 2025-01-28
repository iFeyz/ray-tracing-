use crate::ray::ray::Ray;
use crate::ray::vec3::Point3;
use crate::ray::hittable::{Hittable, HitRecord};
use crate::ray::vec3::dot;
use crate::utils::interval::Interval;
use crate::object::material::Material;
use std::sync::Arc;


pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<dyn Material>) -> Self {
        Sphere { center, radius, material }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - ray.origin();
        let a = ray.direction().length_squared();
        let h = dot(ray.direction(), oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = discriminant.sqrt();
        let mut root = (h - sqrt_discriminant) / a;
        if root < ray_t.min || root > ray_t.max {
            root = (h + sqrt_discriminant) / a;
            if root < ray_t.min || root > ray_t.max {
                return false;
            }
        }

        rec.t = Interval::new(root, root);
        rec.p = ray.at(rec.t.min);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.material = Some(self.material.clone());
        true
    }
}