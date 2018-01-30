
use std::ops::Add;
use std::ops::Mul;

pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 {x: x, y: y, z: z}
    }
    pub fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
    pub fn normalize(&mut self) -> Vec3 {
        let l: f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        Vec3 {x: self.x, y: self.y, z: self.z}
    }
}

// overload
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}


