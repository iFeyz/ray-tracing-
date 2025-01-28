use std::ops;
use std::fmt;
use crate::utils::utils::{random_double, random_double_range};
#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    vec: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { vec: [x, y, z] }
    }

    pub fn x(&self) -> f32 { self.vec[0] }
    pub fn y(&self) -> f32 { self.vec[1] }
    pub fn z(&self) -> f32 { self.vec[2] }

    pub fn length_squared(&self) -> f32 {
        self.dot(self)
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    pub fn rotate_x(&self, theta: f32) -> Vec3 {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Vec3::new(
            self.x(),
            self.y() * cos_theta - self.z() * sin_theta,
            self.y() * sin_theta + self.z() * cos_theta
        )
    }

    pub fn rotate_y(&self, theta: f32) -> Vec3 {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Vec3::new(
            self.x() * cos_theta + self.z() * sin_theta,
            self.y(),
            -self.x() * sin_theta + self.z() * cos_theta
        )
    }

    pub fn rotate_z(&self, theta: f32) -> Vec3 {
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        Vec3::new(
            self.x() * cos_theta - self.y() * sin_theta,
            self.x() * sin_theta + self.y() * cos_theta,
            self.z()
        )
    }
}
// Opeator overloading
impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3::new(-self.vec[0], -self.vec[1], -self.vec[2])
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new(self.vec[0] + other.vec[0], self.vec[1] + other.vec[1], self.vec[2] + other.vec[2])
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new(self.vec[0] - other.vec[0], self.vec[1] - other.vec[1], self.vec[2] - other.vec[2])
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, scalar: f32) -> Vec3 {
        Vec3::new(self.vec[0] * scalar, self.vec[1] * scalar, self.vec[2] * scalar)
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, scalar: f32) -> Vec3 {
        Vec3::new(self.vec[0] / scalar, self.vec[1] / scalar, self.vec[2] / scalar)
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.vec[0] += other.vec[0];
        self.vec[1] += other.vec[1];
        self.vec[2] += other.vec[2];
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, scalar: f32) {
        self.vec[0] *= scalar;
        self.vec[1] *= scalar;
        self.vec[2] *= scalar;
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, scalar: f32) {
        *self *= 1.0 / scalar;
    }
}

//Indexing
impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.vec[index]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.vec[index]
    }
}
// Utility functions
impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.vec[0] * other.vec[0] + self.vec[1] * other.vec[1] + self.vec[2] * other.vec[2]
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3::new(
            self.vec[1] * other.vec[2] - self.vec[2] * other.vec[1],
            self.vec[2] * other.vec[0] - self.vec[0] * other.vec[2],
            self.vec[0] * other.vec[1] - self.vec[1] * other.vec[0]
        )
    }

    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }

    pub fn random_min_max(min: f32, max: f32) -> Vec3 {
        Vec3::new(random_double_range(min, max), random_double_range(min, max), random_double_range(min, max))
    }

    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_min_max(-1.0, 1.0);
            let lensq = p.length_squared();
            if lensq <= 1.0 && lensq > 1e-16 {
                return p / lensq.sqrt();
            }
        }
    }

    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_sphere = Vec3::random_unit_vector();
        if dot(on_unit_sphere, *normal) > 0.0 {
            on_unit_sphere
        } else {
            -on_unit_sphere
        }
    }

    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        self.vec[0].abs() < s && self.vec[1].abs() < s && self.vec[2].abs() < s
    }

    pub fn reflect(&self, n: &Vec3) -> Vec3 {
        *self - 2.0 * dot(*self, *n) * *n
    }

    pub fn refract(&self, n: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = f32::min(dot(-*self, *n), 1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * *n);
        let r_out_parallel = -((1.0 - r_out_perp.length_squared()).abs().sqrt()) * *n;
        r_out_perp + r_out_parallel
    }
}


// Display formatting
impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.vec[0], self.vec[1], self.vec[2])
    }
}

// Type alias for point3
pub type Point3 = Vec3;

// Add this implementation for f32 * Vec3
impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;
    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3::new(self * vec.vec[0], self * vec.vec[1], self * vec.vec[2])
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, other: Vec3) -> Vec3 {
        Vec3::new(self.vec[0] * other.vec[0], self.vec[1] * other.vec[1], self.vec[2] * other.vec[2])
    }
}

pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.vec[0] * v.vec[0] + u.vec[1] * v.vec[1] + u.vec[2] * v.vec[2]
}