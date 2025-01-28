use crate::ray::ray::Ray;
use crate::ray::vec3::{Vec3, Point3};
use crate::ray::hittable::{Hittable, HitRecord};
use crate::utils::interval::Interval;
use std::sync::Arc;
use crate::object::material::Material;

pub struct Triangle {
    pub v0: Point3,
    pub v1: Point3,
    pub v2: Point3,
    pub material: Arc<dyn Material>,
}

impl Hittable for Triangle {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;
        let h = ray.direction().cross(&e2);
        let a = e1.dot(&h);

        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = ray.origin() - self.v0;
        let u = f * s.dot(&h);

        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = s.cross(&e1);
        let v = f * ray.direction().dot(&q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * e2.dot(&q);

        if t < ray_t.min || t > ray_t.max {
            return false;
        }

        rec.t = Interval::new(t, t);
        rec.p = ray.at(rec.t.min);
        rec.set_face_normal(ray, e1.cross(&e2).unit_vector());
        rec.material = Some(self.material.clone());
        true
    }
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: Arc<dyn Material>) -> Self {
        Triangle { v0, v1, v2, material }
    }
} 