use std::ops;
use std::fmt;


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


 pub fn dot(u: Vec3, v: Vec3) -> f32 {
    u.vec[0] * v.vec[0] + u.vec[1] * v.vec[1] + u.vec[2] * v.vec[2]
}