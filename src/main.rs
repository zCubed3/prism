mod tests;
mod math;

use math::vector3::*;
use math::ray::*;
use crate::math::vector::Vector;

const RT_WIDTH: usize = 64;
const RT_HEIGHT: usize = 32;
const RT_ORTHO_SIZE: f32 = 1f32;

fn main() {
    // Simple Triangle intersection test
    let p1 = Vector3::from_array([-0.5f32, -0.5f32, 0f32]);
    let p2 = Vector3::from_array([0.5f32, -0.5f32, 0f32]);
    let p3 = Vector3::from_array([0f32, 0.5f32, 0f32]);

    let origin = Vector3::from_array([0f32, 0f32, -1f32]);
    let direction = Vector3::from_array([0f32, 0f32, 1f32]).normalize();

    for y in 0 .. RT_HEIGHT + 1 {
        let y_coord = -(((y as f32 / RT_HEIGHT as f32) - 0.5f32) * 2.0f32 * RT_ORTHO_SIZE);

        for x in 0 .. RT_WIDTH + 1 {
            let x_coord = ((x as f32 / RT_WIDTH as f32) - 0.5f32) * 2.0f32 * RT_ORTHO_SIZE;

            let offset = Vector3::from_array([x_coord, y_coord, 0f32]);
            let ray = Ray3D::new(origin + offset, direction);

            let result = ray.intersect_triangle((p1, p2, p3));

            if result.is_some() {
                print!("X");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}
