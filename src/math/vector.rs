#![allow(unused)]
#![allow(dead_code)]

use std::ops::*;
use std::cmp::*;
use std::fmt::*;

//
// Delegations (allows us to verify components can work!)
//
pub trait SqrtDelegate {
    fn sqrt_delegate(&self) -> Self;
}

// Constraint for acceptable VectorElements
// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
pub trait VectorComponent:
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
    AddAssign + SubAssign + MulAssign + DivAssign +
    Neg<Output=Self> +
    PartialEq +
    SqrtDelegate +
    Clone + Copy + Default + Display
    where Self: Sized {

}

// A Vector is a type with a strict requirement for components, it is a wrapper around a slice!
// By default VectorComponent isn't implemented! We've implemented it for floating point types inside of component_impls.rs!
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<T: VectorComponent, const COUNT: usize> {
    pub data: [T; COUNT],
}

// Subtypes can declare a better "new" function
impl<T: VectorComponent, const COUNT: usize> Vector<T, COUNT> {
    pub fn from_array(d: [T; COUNT]) -> Self {
        Vector { data: d }
    }

    pub fn from_single(d: T) -> Self {
        Vector { data: [d; COUNT] }
    }

    // Sum != Magnitude!
    pub fn sum(&self) -> T {
        let mut sum = T::default();

        self.iter().for_each(|x| {
            sum += *x
        });

        sum
    }

    // Remember: Magnitude != Sum!
    pub fn magnitude(&self) -> T {
        self.dot(*self).sqrt_delegate()
    }

    pub fn length(&self) -> T {
        self.magnitude()
    }

    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    pub fn cross(&self, rhs : Self) -> Self {
        // Unless 4D or 2D cross products are a thing, we only support 3D cross products!
        // Sorry for such a weird hard coded quirk!
        return if COUNT == 3 {
            let mut p = Self::default();

            p[0] = self[1] * rhs[2] - self[2] * rhs[1];
            p[1] = self[2] * rhs[0] - self[0] * rhs[2];
            p[2] = self[0] * rhs[1] - self[1] * rhs[0];

            p
        } else {
            panic!("Cross products are only supported for 3 dimensional vectors! (Your vector has {} components!)", COUNT);
        }
    }

    pub fn dot(&self, rhs : Self) -> T {
        let mut d = T::default();

        for c in 0 .. COUNT {
            d += self[c] * rhs[c];
        }

        d
    }
}

//
// Default
//
impl<T: VectorComponent, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        Self { data: [T::default(); COUNT] }
    }
}

//
// Deref
//
impl<T: VectorComponent, const COUNT: usize> Deref for Vector<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: VectorComponent, const COUNT: usize> DerefMut for Vector<T, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

//
// Formatting Traits
//
impl<T: VectorComponent, const COUNT: usize> Debug for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}, {}> {{\n", std::any::type_name::<T>(), COUNT).expect("Failed to write!");

        for c in 0 .. COUNT {
            write!(f, "\t[{}] = {}\n", c, self[c]).expect("Failed to write!");
        }

        write!(f, "}}").expect("Failed to write!");

        Ok(())
    }
}

impl<T: VectorComponent, const COUNT: usize> Display for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "<").expect("Failed to write!");

        let mut first = true;
        for c in self.data {
            if !first {
                write!(f, ", ").expect("Failed to write!");
            }

            write!(f, "{c}").expect("Failed to write!");
            first = false;
        }
        write!(f, ">").expect("Failed to write!");

        Ok(())
    }
}

//
// Component Math Traits
//
macro_rules! component_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: VectorComponent, const COUNT: usize> $op<T> for Vector<T, COUNT> {
            fn $func(&mut self, rhs: T) {
                for c in 0 .. COUNT {
                    self[c] $call rhs;
                }
            }
        }
    };
}

macro_rules! component_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: VectorComponent, const COUNT: usize> $op<T> for Vector<T, COUNT> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                let mut prod = self;

                for c in 0 .. COUNT {
                    prod[c] $call rhs;
                }

                prod
            }
        }
    };
}

component_op_assign!(AddAssign, add_assign, +=);
component_op_assign!(SubAssign, sub_assign, -=);
component_op_assign!(MulAssign, mul_assign, *=);
component_op_assign!(DivAssign, div_assign, /=);

component_op!(Add, add, +=);
component_op!(Sub, sub, -=);
component_op!(Mul, mul, *=);
component_op!(Div, div, /=);

//
// Vector math traits
//
macro_rules! vector_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: VectorComponent, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
            fn $func(&mut self, rhs: Self) {
                for c in 0 .. COUNT {
                    self[c] $call rhs[c];
                }
            }
        }
    };
}

macro_rules! vector_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: VectorComponent, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
            type Output = Self;

            fn $func(self, rhs: Self) -> Self::Output {
                let mut prod = self;

                for c in 0 .. COUNT {
                    prod[c] $call rhs[c];
                }

                prod
            }
        }
    };
}

vector_op_assign!(AddAssign, add_assign, +=);
vector_op_assign!(SubAssign, sub_assign, -=);
vector_op_assign!(MulAssign, mul_assign, *=);
vector_op_assign!(DivAssign, div_assign, /=);

vector_op!(Add, add, +=);
vector_op!(Sub, sub, -=);
vector_op!(Mul, mul, *=);
vector_op!(Div, div, /=);

//
// Vector negation
//
impl<T: VectorComponent, const COUNT: usize> Neg for Vector<T, COUNT> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        let mut d = self.clone();

        for c in 0 .. COUNT {
            d[c] = -d[c];
        }

        d
    }
}

//
// Vector comparison
//
impl<T: VectorComponent, const COUNT: usize> PartialEq for Vector<T, COUNT> {
    fn eq(&self, other: &Self) -> bool {
        for c in 0 .. COUNT {
            if self[c] != other[c] {
                return false;
            }
        }

        true
    }
}

// By default into operators aren't implemented, but there is a macro for that!
// Use vector_into_vector!() to implement it for other vector types!
#[macro_export]
macro_rules! vector_from_vector {
    ($from_count:literal, $into_count:literal, $t:ty) => {
        impl From<Vector<$t, $from_count>> for Vector<$t, $into_count> {
            fn from(rhs: Vector<$t, $from_count>) -> Self {
                let mut o = Vector::<$t, $into_count>::default();

                for c in 0 .. min($into_count, $from_count) {
                    o[c] = rhs[c];
                }

                o
            }
        }
    };
}

//
// VectorComponents for float types
//
impl VectorComponent for f32 {}
impl SqrtDelegate for f32 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}

impl VectorComponent for f64 {}
impl SqrtDelegate for f64 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}

//
// Common vector types
//
pub mod common {
    use super::*;

    //
    // Vector2
    //
    pub type Vector2F32 = Vector<f32, 2>;
    pub type Vector2 = Vector2F32;
    pub type Vec2 = Vector2;
    pub type Vector2D = Vector2;
    pub type Vec2D = Vector2;

    pub type Vector2F64 = Vector<f64, 2>;
    pub type HiVector2 = Vector2F64;
    pub type HiVec2 = HiVector2;
    pub type HiVector2D = HiVector2;
    pub type HiVec2D = HiVector2;

    //
    // Vector3
    //
    pub type Vector3F32 = Vector<f32, 3>;
    pub type Vector3 = Vector3F32;
    pub type Vec3 = Vector3;
    pub type Vector3D = Vector3;
    pub type Vec3D = Vector3;

    pub type Vector3F64 = Vector<f64, 3>;
    pub type HiVector3 = Vector3F64;
    pub type HiVec3 = Vector3;
    pub type HiVector3D = Vector3;
    pub type HiVec3D = Vector3;

    //
    // Vector4
    //
    pub type Vector4F32 = Vector<f32, 4>;
    pub type Vector4 = Vector4F32;
    pub type Vec4 = Vector4;
    pub type Vector4D = Vector4;
    pub type Vec4D = Vector4;

    pub type Vector4F64 = Vector<f64, 4>;
    pub type HiVector4 = Vector4F64;
    pub type HiVec4 = HiVector4;
    pub type HiVector4D = HiVector4;
    pub type HiVec4D = HiVector4;

    //
    // Common cast implementations
    //

    // Vector2
    vector_from_vector!(2, 3, f32);
    vector_from_vector!(2, 4, f32);

    vector_from_vector!(2, 3, f64);
    vector_from_vector!(2, 4, f64);

    // Vector3
    vector_from_vector!(3, 2, f32);
    vector_from_vector!(3, 4, f32);

    vector_from_vector!(3, 2, f64);
    vector_from_vector!(3, 4, f64);

    // Vector4
    vector_from_vector!(4, 2, f32);
    vector_from_vector!(4, 3, f32);

    vector_from_vector!(4, 2, f64);
    vector_from_vector!(4, 3, f64);

    //
    // Additional implementations
    //

    // Vector2
    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self::from_array([x, y])
        }
    }

    impl HiVector2 {
        pub fn new(x: f64, y: f64) -> Self {
            Self::from_array([x, y])
        }
    }

    // Vector3
    impl Vector3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self::from_array([x, y, z])
        }
    }

    impl HiVector3 {
        pub fn new(x: f64, y: f64, z: f64) -> Self {
            Self::from_array([x, y, z])
        }
    }

    // Vector4
    impl Vector4 {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self::from_array([x, y, z, w])
        }
    }

    impl HiVector4 {
        pub fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
            Self::from_array([x, y, w, z])
        }
    }
}