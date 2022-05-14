mod math;

use math::vector3::*;

fn main() {
    let mut v1 = Vector3::default();
    let mut v2 = Vector3::default();
    let mut v3 = Vector3::default();

    v1[0] = 1f32;

    v2[1] = 1f32;

    v3[0] = 1f32;
    v3[1] = 1f32;

    let c = v1.cross(v2);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 X v2 = {}", c);
    println!("v3 = {} -> |v3| = {}", v3, v3.normalize());
}
