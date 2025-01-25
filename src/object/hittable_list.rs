use crate::ray::ray::Ray;
use crate::ray::hittable::{Hittable, HitRecord};
use crate::utils::interval::Interval;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

// Implémentation du trait Hittable pour HittableList
impl Hittable for HittableList {
    fn hit(&self, ray: Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for object in self.objects.iter() {
            if object.hit(ray, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                if temp_rec.t.min < closest_so_far {
                    hit_anything = true;
                    closest_so_far = temp_rec.t.min;
                    *rec = temp_rec.clone();
                }
            }
        }
        hit_anything
    }
}

