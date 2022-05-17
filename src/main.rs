mod math;
mod perf;
mod rendering;

use std::io::Write;
use math::vector::common::*;
use math::matrix::common::*;
use math::ray::*;

use std::time;

const RT_WIDTH: usize = 32;
const RT_HEIGHT: usize = 16;
const RT_ORTHO_SIZE: f32 = 1f32;

fn sphere_sdf(p: Vector3, r: f32) -> f32 {
    p.magnitude() - r
}

fn donut_sdf(p: Vector3, (r1, r2) : (f32, f32)) -> f32 {
    let y = Vector3::from_array([p[0], 0f32, p[2]]);
    let q1 = y.magnitude() - r1;
    let q2 = p[1];

    Vector3::from_array([q1, q2, 0f32]).magnitude() - r2
}

fn scene_sdf(s: Vector3) -> f32 {
    //sphere_sdf(s, 0.2f32)
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
    // http://paulbourke.net/dataformats/asciiart/
    //let ascii_map = ".:-=+*#%@";
    let ascii_map = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

    let mut last_instant = time::Instant::now();
    let mut time: f32 = 0f32;

    // Shitty blanking system
    for _ in 0 .. 256 {
        println!("                                                            ")
    }

    loop {
        let mut offset = Vector3::default();

        offset[0] = time.sin() * 0.5f32;
        offset[1] = time.cos() * 0.1f32;
        offset[2] -= 0.6f32;

        //let mat_v = Matrix4x4::rotation(Vector3::new(time.cos() * 0.1f32, time.sin(), 0f32) * 0.5f32);
        let mat_v = Matrix4x4::rotate_x(0f32);
        let mat_p = Matrix4x4::perspective(60f32.to_radians(), 1.0f32, 0.1f32, 1000f32);

        let mat_vp = mat_p * mat_v;

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
                //let direction = Vector3::from_array([persp_x, persp_y, 1f32]).normalize();

                let ray = mat_vp * Vector4::new(persp_x, persp_y, -1f32, 0f32);
                let direction = Vector3::from(ray).normalize();

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
                        i = (1f32 - (n.dot(v).max(0f32))).powf(3f32);

                        intersect = true;
                        break;
                    }

                    t += r;
                }

                if intersect && ray[3] > 0f32 {
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
        let delta = (now - last_instant).as_secs_f32();

        time += delta;

        println!("Refresh ~= {}", 1f32 / delta);

        last_instant = now;
    }
}