mod tests;
mod math;
mod perf;

use std::io::Write;
use std::time;
use math::vector::common::*;
use math::ray::*;

const RT_WIDTH: usize = 80;
const RT_HEIGHT: usize = 40;
const RT_ORTHO_SIZE: f32 = 1f32;

fn sphere_sdf(s: Vector3, r: f32) -> f32 {
    s.magnitude() - r
}

fn donut_sdf(s: Vector3, (r1, r2) : (f32, f32)) -> f32 {
    let y = Vector3::from_array([s[0], 0f32, s[2]]);
    let q1 = y.magnitude() - r1;
    let q2 = s[1];

    Vector3::from_array([q1, q2, 0f32]).magnitude() - r2
}

fn scene_sdf(s: Vector3) -> f32 {
    //sphere_sdf(s, 0.5f32)
    donut_sdf(Vector3::from_array([s[0], s[2], s[1]]), (0.5, 0.1))
}

fn normal_sdf(s: Vector3) -> Vector3 {
    let e = Vector3::from_single(0.01f32);

    const X: f32 = 1.0f32;
    const Y: f32 = -1.0f32;

    let k1 = Vector3::from_array([X, Y, Y]);
    let k2 = Vector3::from_array([Y, Y, X]);
    let k3 = Vector3::from_array([Y, X, Y]);
    let k4 = Vector3::from_array([X, X, X]);

    k1 * scene_sdf(k1 + s * e) +
    k2 * scene_sdf(k2 + s * e) +
    k3 * scene_sdf(k3 + s * e) +
    k4 * scene_sdf(k4 + s * e)
}

// SDF main
fn main() {
    let ascii_map = ".:-=+*#%@";

    let mut last_instant = time::Instant::now();
    let mut time: f32 = 0f32;

    // Shitty blanking system
    for _ in 0 .. 256 {
        println!("                                                            ")
    }

    loop {
        let mut offset = Vector3::default();

        offset[0] = time.cos();
        //offset[1] = time.cos();
        offset *= 0.5f32;

        offset[2] -= time.sin().abs() + 0.15f32;

        print!("\x1b[0;0H");
        std::io::stdout().flush();

        let _sdf_time = perf::scoped_stopwatch::ScopedStopwatch::new_begin("SDF".to_string());
        for y in 0..RT_HEIGHT + 1 {
            let v = y as f32 / RT_HEIGHT as f32;

            let persp_y = -((v - 0.5f32) * 2f32);
            //let ortho_y = v * RT_ORTHO_SIZE;

            for x in 0..RT_WIDTH + 1 {
                let u = (x as f32 / RT_WIDTH as f32);

                let persp_x = (u - 0.5f32) * 2f32;
                //let ortho_x = u * RT_ORTHO_SIZE;

                //let origin = Vector3::from_array([ortho_x, ortho_y, 0f32]) + offset;
                let origin = offset;

                //let direction = Vector3::from_array([0f32, 0f32, 1f32]).normalize();
                let direction = Vector3::from_array([persp_x, persp_y, 1f32]).normalize();

                let mut intersect = false;
                let mut i = 0.0f32;

                let mut t = 0.0;
                while t < 10.0 {
                    let s = origin + direction * t;

                    let r = scene_sdf(s);

                    if r < 0.001f32 {
                        let n = normal_sdf(s).normalize();

                        i = n.dot(Vector3::from_array([time.sin(), time.cos(), -1f32]).normalize());

                        let v = (s - origin).normalize();
                        i = (n.dot(v).max(0f32));

                        intersect = true;
                        break;
                    }

                    t += r;
                }

                if intersect {
                    let m = (ascii_map.len() - 1) as f32;
                    let c = (i.min(1.0).max(0.0) * m).ceil() as usize;

                    //print!("{} ", c);
                    print!("{}", ascii_map.chars().nth(c).unwrap());
                } else {
                    print!(" ");
                }
            }

            println!();
        }

        let now = time::Instant::now();
        time += (now - last_instant).as_secs_f32();

        last_instant = now;
    }
}

// RT main
/*
fn main() {
    let _prog_time = perf::scoped_stopwatch::ScopedStopwatch::new_begin("RUN".to_string());

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

            let mut offset = Vector3::from_array([x_coord, y_coord, 0f32]);
            offset += origin;

            let ray = Ray3D::new(offset, direction);

            let result = ray.intersect_triangle((p1, p2, p3));

            if result.is_some() {
                print!("*");
            } else {
                print!(" ");
            }
        }

        println!();
    }
}
*/