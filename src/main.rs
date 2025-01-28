use ray::ray::vec3::{Vec3, Point3};
use ray::ray::color::write_color;
use ray::ray::ray::Ray;
use ray::object::sphere::Sphere;
use ray::object::pyramid::Pyramid;
use ray::object::hittable_list::HittableList;
use ray::utils::camera::Camera;
use ray::object::material::Lambertian;
use std::sync::Arc;
use ray::object::material::Metal;
use ray::object::material::Dielectric;

fn main() {
    // World
    let mut world = HittableList::new();

    // Matériaux avec des couleurs plus vives
    let material_ground = Arc::new(Lambertian { 
        albedo: Vec3::new(1.0, 1.0, 1.0),  // Vert vif pour le sol
        fuzz: 0.0 
    });

    let material_left = Arc::new(Metal { 
        albedo: Vec3::new(0.9, 0.1, 0.1),  // Rouge métallique
        fuzz: 0.3 
    });

    let material_right = Arc::new(Dielectric { 
        albedo: Vec3::new(0.8, 0.8, 1.0),  // Bleu clair pour le verre
        ir: 1.0 / 1.3, 
        fuzz: 0.0 
    });

    let material_pyramid = Arc::new(Metal { 
        albedo: Vec3::new(0.0, 0.0, 1.0),  // Or (couleur dorée)
        fuzz: 0.2 
    });

    let material_gold = Arc::new(Metal { 
        albedo: Vec3::new(0.9, 0.7, 0.1),  // Or (couleur dorée)
        fuzz: 0.2 
    });
    
    let material_silver = Arc::new(Metal { 
        albedo: Vec3::new(0.9, 0.9, 0.9),  // Argent (couleur argentée)
        fuzz: 0.2 
    });

    let material_mirror = Arc::new(Metal { 
        albedo: Vec3::new(0.9, 0.9, 0.9),  // Argent (couleur argentée)
        fuzz: 0.0 
    });

    let material_glass = Arc::new(Dielectric { 
        albedo: Vec3::new(0.9, 0.9, 0.9),  // Argent (couleur argentée)
        ir: 1.0 / 1.3, 
        fuzz: 0.0 
    });

    let material_purple = Arc::new(Lambertian {
        albedo: Vec3::new(0.5, 0.0, 0.5),  
        
        fuzz: 0.0
    });

    let material_cyan = Arc::new(Metal {
        albedo: Vec3::new(0.0, 0.8, 0.8),  
        
        fuzz: 0.1
    });

    
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, material_ground)));
    //world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0,-1.0), 0.5, material_purple)));
    //world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0,-1.0), 0.5, material_cyan)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0,-1.5), 0.5, material_glass)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 0.0,-1.0), 0.5, material_mirror)));
    world.add(Box::new(Sphere::new(Point3::new(1.0, 0.0,-2.0), 0.5, material_gold)));
    world.add(Box::new(Sphere::new(Point3::new(-2.0, 0.0,-2.0), 0.5, material_silver)));
    
    
    //world.add(Box::new(Pyramid::new(
        //Point3::new(0.0, 0.0, -2.0),
        //1.0,
        //1.5,
        //45.0_f32.to_radians(),
        //30.0_f32.to_radians(),
        //0.0,
      //  material_pyramid
    //)));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 500;
    let samples_per_pixel = 100;
    let look_from = Point3::new(-2.0, 2.0, 1.0);
    let look_at = Point3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 0.5, 0.0);
    let mut camera = Camera::new(aspect_ratio, image_width, samples_per_pixel, look_from, look_at, vup);

    // Render
    camera.render(&world);
}


 