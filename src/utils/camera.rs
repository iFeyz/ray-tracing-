use crate::ray::ray::Ray;
use crate::ray::vec3::{Vec3, Point3};
use crate::ray::hittable::{Hittable, HitRecord};
use crate::utils::interval::Interval;
use crate::ray::color::write_color;
use crate::utils::utils::random_double;
use minifb::{Window, WindowOptions, Key};

pub struct Camera {
    pub aspect_ratio : f32,
    pub image_width : i32,
    pub samples_per_pixel : i32,
    pub max_depth : i32,
    pub image_height : i32,
    pub center : Point3,
    pub pixel_sample_scale : f32,
    pub pixel_delta_u: Vec3,
    pub pixel_delta_v: Vec3,
    pub pixel00_loc: Point3,
    pub vfov: f32,
    pub look_from: Point3,
    pub look_at: Point3,
    pub vup: Vec3,
    buffer: Vec<u32>,
    window: Option<Window>,
}

impl Camera {
    pub fn new(aspect_ratio: f32, image_width: i32, samples_per_pixel: i32 , look_from: Point3, look_at: Point3, vup: Vec3) -> Self {
        let image_height = (image_width as f32 / aspect_ratio) as i32;
        let buffer_size = (image_width * image_height) as usize;
        let mut window = Window::new(
            "Ray Tracer Preview",
            image_width as usize,
            image_height as usize,
            WindowOptions::default(),
        ).unwrap();
        
        window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

        Camera {
            max_depth: 20,
            aspect_ratio,
            image_width,
            samples_per_pixel,
            image_height,
            vfov: 40.0,
            look_from,
            look_at,
            vup,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_sample_scale: 0.0,
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel00_loc: Point3::new(0.0, 0.0, 0.0),
            buffer: vec![0; buffer_size],
            window: Some(window),
        }
    }

    pub fn intialize(&mut self) {
        self.image_height = (self.image_width as f32 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 { 1 } else { self.image_height };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f32;
        // Viewport dimensions
        self.center = self.look_from;
        let focal_length = (self.look_from - self.look_at).length();
        let theta = self.vfov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * self.aspect_ratio;
        //Calculte the u , v , w vectors 
        let w = (self.look_from - self.look_at).unit_vector();
        let u = self.vup.cross(&w).unit_vector();
        let v = w.cross(&u);
        // Calculate the vectors horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * -v;
        // Calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f32;
        self.pixel_delta_v = viewport_v / self.image_height as f32;
        // Calculate the location of the upper left pixel
        let viewport_upper_left = self.center - (focal_length * w) - (viewport_u / 2.0) - (viewport_v / 2.0);
        self.pixel00_loc = viewport_upper_left + (self.pixel_delta_u + self.pixel_delta_v) * 0.5;
    }

    pub fn ray_color(ray: Ray, world: &dyn Hittable, depth: i32) -> Vec3 {
        if depth <= 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }
        let mut rec = HitRecord::new();
        if world.hit(ray, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let mut scattered = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Vec3::new(0.0, 0.0, 0.0);
            if let Some(ref material) = rec.material {
                if material.scatter(&ray, &rec, &mut attenuation, &mut scattered) {
                    return Camera::ray_color(scattered, world, depth - 1) * attenuation;
                }
            }
            return Vec3::new(0.0, 0.0, 0.0);
        }

        let unit_direction = ray.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Vec3::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3::new(0.5, 0.7, 1.0) * a
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc + (i as f32 * self.pixel_delta_u + j as f32 * self.pixel_delta_v) + offset;
        let ray_direction = pixel_sample - self.center;
        Ray::new(self.center, ray_direction)
    }

    fn sample_square(&self) -> Vec3 {
        let px = -0.5 + random_double();
        let py = -0.5 + random_double();
        Vec3::new(px, py, 0.0) * self.pixel_sample_scale
    }

    fn update_pixel(&mut self, x: i32, y: i32, color: Vec3) {
        let idx = (y * self.image_width + x) as usize;
        let r = (color.x() * 255.0) as u32;
        let g = (color.y() * 255.0) as u32;
        let b = (color.z() * 255.0) as u32;
        self.buffer[idx] = (r << 16) | (g << 8) | b;
        
        // Mettre à jour la fenêtre tous les N pixels
        if idx % 1000 == 0 {
            if let Some(window) = &mut self.window {
                window.update_with_buffer(&self.buffer, 
                    self.image_width as usize, 
                    self.image_height as usize
                ).unwrap();
            }
        }
    }

    pub fn render(&mut self, world: &dyn Hittable) {
        self.intialize();
        println!("P3");
        println!("{} {}", self.image_width, self.image_height);
        println!("255");

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
                for _ in 0..self.samples_per_pixel {
                    let r: Ray = self.get_ray(i, j);
                    pixel_color += Camera::ray_color(r, world, self.max_depth);
                }
                
                // Gamma correction et normalisation
                let scale = 1.0 / self.samples_per_pixel as f32;
                let r = (pixel_color.x() * scale).sqrt();
                let g = (pixel_color.y() * scale).sqrt();
                let b = (pixel_color.z() * scale).sqrt();
                
                let final_color = Vec3::new(r, g, b);
                self.update_pixel(i, j, final_color);
                write_color(pixel_color, self.samples_per_pixel);
            }
        }

        // Afficher la fenêtre finale
        if let Some(window) = &mut self.window {
            while window.is_open() && !window.is_key_down(Key::Escape) {
                window.update_with_buffer(&self.buffer, 
                    self.image_width as usize, 
                    self.image_height as usize
                ).unwrap();
            }
        }
    }
}