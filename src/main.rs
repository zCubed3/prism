mod tests;
mod math;

use math::vector3::*;

fn main() {
    let v1 = Vector3::from_array([1f32, 0f32, 0f32]);
    let v2 = Vector3::from_array([0f32, 1f32, 0f32]);
    let v3 = Vector3::from_array([0.5f32, 1f32, 1f32]);

    let c = v1.cross(v2);

    println!("v1 = {}", v1);
    println!("v2 = {}", v2);
    println!("v1 X v2 = {}", c);
    println!("v3 = {} -> |v3| = {}", v3, v3.normalize());
}
