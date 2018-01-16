
extern crate webgl_simple_math;

fn main(){
    let mut v = webgl_simple_math::vector_three::VectorThree::new(1.0, 1.0, 1.0);
    let l: f64 = v.length();
    println!("initial: {} {} {}: {}", v.x, v.y, v.z, l);

    let w: webgl_simple_math::vector_three::VectorThree = v.normalize();
    let l: f64 = w.length();
    println!("normalize: {} {} {}: {}", w.x, w.y, w.z, l);
}





