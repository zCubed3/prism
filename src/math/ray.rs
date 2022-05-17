#![allow(unused)]
#![allow(dead_code)]

//
// 3D Ray type backed by Vector<f32, 3> aka Vector3
//

use crate::math::vector::common::Vector3;

pub struct Ray3D {
    pub origin: Vector3,
    pub direction: Vector3
}

const EPSILON : f32 = 0.0000001f32;

impl Ray3D {
    pub fn new(origin: Vector3, direction: Vector3) -> Self {
        Self { origin, direction }
    }

    pub fn intersect_triangle(self, (p1, p2, p3) : (Vector3, Vector3, Vector3)) -> Option<(f32, f32, f32)> {
        let e1 = p2 - p1;
        let e2 = p3 - p1;

        let h = self.direction.cross(e2);
        let a = e1.dot(h);

        // Is parallel?
        if a > -EPSILON && a < EPSILON {
            return None;
        }

        let f = 1.0 / a;
        let s = self.origin - p1;
        let u = f * s.dot(h);

        if u < 0.0f32 || u > 1.0f32 {
            return None;
        }

        let q = s.cross(e1);
        let v = f * self.direction.dot(q);

        if v < 0.0f32 || u + v > 1.0f32 {
            return None;
        }

        let t = f * e2.dot(q);

        if t > EPSILON {
            return Some((u, v, t));
        }

        return None;
    }
}