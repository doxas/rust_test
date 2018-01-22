
extern crate webgl_simple_math;

use webgl_simple_math::vector::vector_three::Vec3;

fn main(){
    let mut v = Vec3::new(1.0, 1.0, 1.0);
    let l: f64 = v.length();
    println!("initial: {} {} {}: {}", v.x, v.y, v.z, l);

    let w: Vec3 = v.normalize();
    let l: f64 = w.length();
    println!("normalize: {} {} {}: {}", w.x, w.y, w.z, l);

    let x: Vec3 = v + w;
    println!("normalize: {} {} {}: {}", x.x, x.y, x.z, 0.0);
}





