use crate::ray::ray::Ray;
use crate::ray::vec3::Vec3;
use crate::ray::hittable::HitRecord;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Vec3, // Couleur de diffusion
    pub fuzz: f32,
}

#[derive(Clone)]
pub struct Dielectric {
    pub albedo: Vec3,
    pub fuzz : f32,
    pub ir : f32,
}

#[derive(Clone)]
pub struct Metal {
    pub albedo: Vec3,
    pub fuzz : f32,
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector() * self.fuzz;
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        *attenuation = Vec3::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = r_in.direction().unit_vector();
        let refracted = unit_direction.refract(&rec.normal, refraction_ratio);
        *scattered = Ray::new(rec.p, refracted);
        true
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Vec3, scattered: &mut Ray) -> bool {
        let reflected = r_in.direction().reflect(&rec.normal);
        *scattered = Ray::new(rec.p, reflected + Vec3::random_unit_vector() * self.fuzz);
        *attenuation = self.albedo;
        true
    }
}


