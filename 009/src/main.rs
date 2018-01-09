
// struct
struct VectorThree {
    x: f64,
    y: f64,
    z: f64,
}

// method
impl VectorThree {
    fn new(x: f64, y: f64, z: f64) -> Self {
        VectorThree {x: x, y: y, z: z}
    }
    fn length(&self) -> f64 {
        let l: f64 = self.x * self.x + self.y * self.y + self.z * self.z;
        l.sqrt()
    }
}

fn main(){
    let v = VectorThree::new(1.0, 1.0, 1.0);
    let l: f64 = v.length();
    println!("{}", l);
}


