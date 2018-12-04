
extern crate webgl_simple_math;

use webgl_simple_math::vector::vector_two::Vec2;
use webgl_simple_math::vector::vector_three::Vec3;

fn out_vec2(){
    let mut v = Vec2::new(1.0, 1.0);
    let l: f64 = v.length();
    println!("initial: {} {}: {}", v.x, v.y, l);

    let w: Vec2 = v.normalize();
    let l: f64 = w.length();
    println!("normalize: {} {}: {}", w.x, w.y, l);

    let x: Vec2 = v + w;
    println!("additional: {} {}: {}", x.x, x.y, 0.0);

    let y: Vec2 = v - w;
    println!("subtract: {} {}: {}", y.x, y.y, 0.0);

    let z: Vec2 = v * w;
    println!("multiple: {} {}: {}", z.x, z.y, 0.0);

    let a: Vec2 = v / w;
    println!("dividing: {} {}: {}", a.x, a.y, 0.0);

    let b: f64 = v.dot(&w);
    println!("dot prod: {}", b);

    let c: f64 = v.cross(&w);
    println!("cross prod: {}", c);
}

fn out_vec3(){
    let mut v = Vec3::new(1.0, 1.0, 1.0);
    let l: f64 = v.length();
    println!("initial: {} {} {}: {}", v.x, v.y, v.z, l);

    let w: Vec3 = v.normalize();
    let l: f64 = w.length();
    println!("normalize: {} {} {}: {}", w.x, w.y, w.z, l);

    let x: Vec3 = v + w;
    println!("additional: {} {} {}: {}", x.x, x.y, x.z, 0.0);

    let y: Vec3 = v - w;
    println!("subtract: {} {} {}: {}", y.x, y.y, y.z, 0.0);

    let z: Vec3 = v * w;
    println!("multiple: {} {} {}: {}", z.x, z.y, z.z, 0.0);

    let a: Vec3 = v / w;
    println!("dividing: {} {} {}: {}", a.x, a.y, a.z, 0.0);

    let b: f64 = v.dot(&w);
    println!("dot prod: {}", b);

    let c: Vec3 = v.cross(&w);
    println!("cross prod: {} {} {}", c.x, c.y, c.z);
}

fn main(){
    println!("out-vec2");
    out_vec2();

    println!("out-vec3");
    out_vec3();
}





