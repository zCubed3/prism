#![allow(unused)]
#![allow(dead_code)]

use std::ops::*;
use std::cmp::*;
use std::fmt::*;

//
// Delegations (allows us to verify components can work!)
//

/// Required trait for vector components!
///
/// Because of [Vector::magnitude()], it is necessary to get the square root of the component
/// If your component type can't provide a square root it won't be usable!
///
/// This trait is already implemented for [f32] and [f64]
pub trait SqrtDelegate {
    fn sqrt_delegate(&self) -> Self;
}


// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
/// Strict trait for constraining what types can be used as vector components
///
/// This trait is already implemented for [f32] and [f64]
pub trait VectorComponent:
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
    AddAssign + SubAssign + MulAssign + DivAssign +
    Neg<Output=Self> +
    PartialEq +
    SqrtDelegate +
    Clone + Copy + Default + Display
    where Self: Sized {

}

///
/// Configurable vector type for usage with Vector math
///
/// A vector is simply a wrapper for an array of the given component type and count
/// Supports any component that can be implemented as a [VectorComponent] trait
///
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<TComponent: VectorComponent, const COUNT: usize> {
    /// The underlying array of the vector, the vector dereferences into this array
    pub data: [TComponent; COUNT],
}

impl<TComponent: VectorComponent, const COUNT: usize> Vector<TComponent, COUNT> {
    /// Creates a new [Vector] by copying the given array into the backing array
    pub fn from_array(array: [TComponent; COUNT]) -> Self {
        Vector { data: array }
    }

    /// Creates a new [Vector] by copying the provided value into each element
    pub fn from_single(value: TComponent) -> Self {
        Vector { data: [value; COUNT] }
    }

    /// Returns the sum of all [VectorComponent]'s within this [Vector]
    pub fn sum(&self) -> TComponent {
        let mut sum = TComponent::default();

        self.iter().for_each(|x| {
            sum += *x
        });

        sum
    }

    /// The length of this [Vector], not to be confused with [Vector::sum]!
    pub fn magnitude(&self) -> TComponent {
        self.dot(*self).sqrt_delegate()
    }

    /// Returns the normalized version of this [Vector]
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    /// Returns the dot product of this [Vector] and another
    pub fn dot(&self, rhs : Self) -> TComponent {
        let mut d = TComponent::default();

        for c in 0 .. COUNT {
            d += self[c] * rhs[c];
        }

        d
    }
}

//
// Default
//
impl<TComponent: VectorComponent, const COUNT: usize> Default for Vector<TComponent, COUNT> {
    fn default() -> Self {
        Self { data: [TComponent::default(); COUNT] }
    }
}

//
// Deref
//
/// Deref to allow the Vector to be treated as its underlying backing array
impl<TComponent: VectorComponent, const COUNT: usize> Deref for Vector<TComponent, COUNT> {
    type Target = [TComponent; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<TComponent: VectorComponent, const COUNT: usize> DerefMut for Vector<TComponent, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

//
// Formatting Traits
//
impl<TComponent: VectorComponent, const COUNT: usize> Debug for Vector<TComponent, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}, {}> {{\n", std::any::type_name::<TComponent>(), COUNT).expect("Failed to write!");

        for c in 0 .. COUNT {
            write!(f, "\t[{}] = {}\n", c, self[c]).expect("Failed to write!");
        }

        write!(f, "}}").expect("Failed to write!");

        Ok(())
    }
}

impl<TComponent: VectorComponent, const COUNT: usize> Display for Vector<TComponent, COUNT> {
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
        impl<TComponent: VectorComponent, const COUNT: usize> $op<TComponent> for Vector<TComponent, COUNT> {
            fn $func(&mut self, rhs: TComponent) {
                for c in 0 .. COUNT {
                    self[c] $call rhs;
                }
            }
        }
    };
}

macro_rules! component_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<TComponent: VectorComponent, const COUNT: usize> $op<TComponent> for Vector<TComponent, COUNT> {
            type Output = Self;

            fn $func(self, rhs: TComponent) -> Self::Output {
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
        impl<TComponent: VectorComponent, const COUNT: usize> $op<Self> for Vector<TComponent, COUNT> {
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
        impl<TComponent: VectorComponent, const COUNT: usize> $op<Self> for Vector<TComponent, COUNT> {
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
impl<TComponent: VectorComponent, const COUNT: usize> Neg for Vector<TComponent, COUNT> {
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
impl<TComponent: VectorComponent, const COUNT: usize> PartialEq for Vector<TComponent, COUNT> {
    fn eq(&self, other: &Self) -> bool {
        for c in 0 .. COUNT {
            if self[c] != other[c] {
                return false;
            }
        }

        true
    }
}

//
// Extension macros
//

/// Macro to provide a [From] implementation for casting this [Vector] into another [Vector]
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

/// Contains commonly used Vector types with additional implementations for ease of use
pub mod common {
    use super::*;

    /// 2 Dimensional Vector
    pub type Vector2 = Vector<f32, 2>;
    vector_from_vector!(2, 3, f32);
    vector_from_vector!(2, 4, f32);

    impl Vector2 {
        pub fn new(x: f32, y: f32) -> Self {
            Self::from_array([x, y])
        }
    }

    /// 3 Dimensional Vector
    pub type Vector3 = Vector<f32, 3>;
    vector_from_vector!(3, 2, f32);
    vector_from_vector!(3, 4, f32);

    impl Vector3 {
        pub fn new(x: f32, y: f32, z: f32) -> Self {
            Self::from_array([x, y, z])
        }

        /// Returns the cross product of the this [Vector] and another
        /// *Only implemented for 3 dimensional vectors due to cross product being 3D specific!*
        pub fn cross(&self, rhs : Self) -> Self {
            Self::from_array([
                self[1] * rhs[2] - self[2] * rhs[1],
                self[2] * rhs[0] - self[0] * rhs[2],
                self[0] * rhs[1] - self[1] * rhs[0]
            ])
        }
    }

    /// 4 Dimensional Vector
    pub type Vector4 = Vector<f32, 4>;
    vector_from_vector!(4, 2, f32);
    vector_from_vector!(4, 3, f32);

    impl Vector4 {
        pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
            Self::from_array([x, y, z, w])
        }
    }
}