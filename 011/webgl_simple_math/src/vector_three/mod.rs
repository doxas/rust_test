
pub struct VectorThree {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl VectorThree {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        VectorThree {x: x, y: y, z: z}
    }
    pub fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
    pub fn normalize(&mut self) -> VectorThree {
        let l: f64 = self.length();
        self.x /= l;
        self.y /= l;
        self.z /= l;
        VectorThree {x: self.x, y: self.y, z: self.z}
    }
}


