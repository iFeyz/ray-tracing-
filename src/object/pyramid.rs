use crate::ray::ray::Ray;
use crate::ray::vec3::{Vec3, Point3};
use crate::ray::hittable::{Hittable, HitRecord};
use crate::utils::interval::Interval;
use std::sync::Arc;
use crate::object::material::Material;
use crate::object::triangle::Triangle;

pub struct Pyramid {
    triangles: Vec<Triangle>,
}

impl Pyramid {
    pub fn new(center: Point3, base_width: f32, height: f32, 
               rotation_x: f32, rotation_y: f32, rotation_z: f32,
               material: Arc<dyn Material>) -> Self {
        let half_width = base_width / 2.0;
        
        // Base points (counter-clockwise from front-right)
        let mut p1 = Point3::new(half_width, 0.0, half_width);
        let mut p2 = Point3::new(-half_width, 0.0, half_width);
        let mut p3 = Point3::new(-half_width, 0.0, -half_width);
        let mut p4 = Point3::new(half_width, 0.0, -half_width);
        let mut apex = Point3::new(0.0, height, 0.0);

        // Appliquer les rotations
        let points = [&mut p1, &mut p2, &mut p3, &mut p4, &mut apex];
        for point in points {
            *point = point.rotate_x(rotation_x);
            *point = point.rotate_y(rotation_y);
            *point = point.rotate_z(rotation_z);
            *point = *point + center;
        }

        let mut triangles = Vec::new();

        // Four sides
        triangles.push(Triangle::new(p1, p2, apex, material.clone())); // Front face
        triangles.push(Triangle::new(p2, p3, apex, material.clone())); // Left face
        triangles.push(Triangle::new(p3, p4, apex, material.clone())); // Back face
        triangles.push(Triangle::new(p4, p1, apex, material.clone())); // Right face

        // Base (made of two triangles)
        triangles.push(Triangle::new(p1, p3, p2, material.clone())); // Base triangle 1
        triangles.push(Triangle::new(p1, p4, p3, material.clone())); // Base triangle 2

        Pyramid { triangles }
    }
}

impl Hittable for Pyramid {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for triangle in &self.triangles {
            if triangle.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t.min;
                *rec = temp_rec.clone();
            }
        }

        hit_anything
    }
} 