#![allow(unused)]
#![allow(dead_code)]

use super::component::Component;

use std::ops::*;
use std::cmp::*;
use std::fmt::*;

///
/// Configurable vector type for usage with Vector math
///
/// A vector is simply a wrapper for an array of the given component type and count
/// Supports any component that can be implemented as a [VectorComponent] trait
///
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<T: Component, const COUNT: usize> {
    /// The underlying array of the vector, the vector dereferences into this array
    pub data: [T; COUNT],
}

impl<T: Component, const COUNT: usize> Vector<T, COUNT> {
    /// Creates a new [Vector] by copying the given array into the backing array
    pub fn from_array(array: [T; COUNT]) -> Self {
        Vector { data: array }
    }

    /// Creates a new [Vector] by copying the provided value into each element
    pub fn from_single(value: T) -> Self {
        Vector { data: [value; COUNT] }
    }

    /// Returns the sum of all [VectorComponent]'s within this [Vector]
    pub fn sum(&self) -> T {
        let mut sum = T::default();

        self.iter().for_each(|x| {
            sum += *x
        });

        sum
    }

    /// The length of this [Vector], not to be confused with [Vector::sum]!
    pub fn magnitude(&self) -> T {
        self.dot(*self).sqrt_delegate()
    }

    /// Returns the normalized version of this [Vector]
    pub fn normalize(&self) -> Self {
        *self / self.magnitude()
    }

    /// Returns the dot product of this [Vector] and another
    pub fn dot(&self, rhs : Self) -> T {
        let mut d = T::default();

        for c in 0 .. COUNT {
            d += self[c] * rhs[c];
        }

        d
    }

    /// Returns a copy of this [Vector] with each component set to their absolute value
    pub fn abs(&self) -> Self {
        let mut a = *self;

        for mut v in *a {
            v = v.abs_delegate();
        }

        a
    }
}

//
// Default
//
impl<T: Component, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        Self { data: [T::default(); COUNT] }
    }
}

//
// Deref
//
/// Deref to allow the Vector to be treated as its underlying backing array
impl<T: Component, const COUNT: usize> Deref for Vector<T, COUNT> {
    type Target = [T; COUNT];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Component, const COUNT: usize> DerefMut for Vector<T, COUNT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

//
// Formatting Traits
//
impl<T: Component, const COUNT: usize> Debug for Vector<T, COUNT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Vector<{}, {}> {{\n", std::any::type_name::<T>(), COUNT).expect("Failed to write!");

        for c in 0 .. COUNT {
            writeln!(f, "\t[{}] = {}", c, self[c]).expect("Failed to write!");
        }

        write!(f, "}}").expect("Failed to write!");

        Ok(())
    }
}

impl<T: Component, const COUNT: usize> Display for Vector<T, COUNT> {
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
        impl<T: Component, const COUNT: usize> $op<T> for Vector<T, COUNT> {
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
        impl<T: Component, const COUNT: usize> $op<T> for Vector<T, COUNT> {
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
        impl<T: Component, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
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
        impl<T: Component, const COUNT: usize> $op<Self> for Vector<T, COUNT> {
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
impl<T: Component, const COUNT: usize> Neg for Vector<T, COUNT> {
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
impl<T: Component, const COUNT: usize> PartialEq for Vector<T, COUNT> {
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
    ($from_count:literal, $into_count:literal) => {
        impl<T: Component> From<Vector<T, $from_count>> for Vector<T, $into_count> {
            fn from(rhs: Vector<T, $from_count>) -> Self {
                let mut o = Vector::<T, $into_count>::default();

                for c in 0 .. min($into_count, $from_count) {
                    o[c] = rhs[c];
                }

                o
            }
        }
    };
}

//
// Common vector types
//

/// Contains commonly used [Vector] aliases with additional implementations for ease of use
pub mod common {
    use super::*;

    /// 2D Vector
    pub type Vector2 = Vector<f32, 2>;
    vector_from_vector!(2, 3);
    vector_from_vector!(2, 4);

    impl<T: Component> Vector<T, 2> {
        pub fn new(x: T, y: T) -> Self {
            Self::from_array([x, y])
        }
    }

    /// 3D Vector
    pub type Vector3 = Vector<f32, 3>;
    vector_from_vector!(3, 2);
    vector_from_vector!(3, 4);

    impl<T: Component> Vector<T, 3> {
        pub fn new(x: T, y: T, z: T) -> Self {
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

    /// 4D Vector (same type as [Quaternion])
    pub type Vector4 = Vector<f32, 4>;
    vector_from_vector!(4, 2);
    vector_from_vector!(4, 3);

    impl<T: Component> Vector<T, 4> {
        pub fn new(x: T, y: T, z: T, w: T) -> Self {
            Self::from_array([x, y, z, w])
        }
    }

    /// Quaternion (same type as [Vector4])
    pub type Quaternion = Vector4;

    impl Quaternion {

    }
}