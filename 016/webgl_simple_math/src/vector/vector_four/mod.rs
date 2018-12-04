
use std::ops::Add;
use std::ops::Sub;
use std::ops::Mul;
use std::ops::Div;

#[derive(Copy, Clone)]
pub struct Vec4 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Vec4 {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Vec4 {x: x, y: y, z: z, w: w}
    }
    pub fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w;
        l.sqrt()
    }
    pub fn dot(&self, vec: &Vec4) -> f64 {
        self.x * vec.x + self.y * vec.y + self.z * vec.z + self.w * vec.w
    }
    pub fn normalize(&mut self) -> Vec4 {
        let l: f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        self.w /= l;
        Vec4 {x: self.x, y: self.y, z: self.z, w: self.w}
    }
}

// overload
impl Add for Vec4 {
    type Output = Vec4;

    fn add(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }
}
impl Sub for Vec4 {
    type Output = Vec4;

    fn sub(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w
        }
    }
}
impl Mul for Vec4 {
    type Output = Vec4;

    fn mul(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
            w: self.w * other.w
        }
    }
}
impl Div for Vec4 {
    type Output = Vec4;

    fn div(self, other: Vec4) -> Vec4 {
        Vec4 {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
            w: self.w / other.w
        }
    }
}


