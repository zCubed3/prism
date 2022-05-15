#![allow(unused)]
#![allow(dead_code)]

use std::ops::*;
use std::cmp::*;
use std::fmt::*;

//
// Delegations
//
pub trait SqrtDelegate {
    fn sqrt_delegate(&self) -> Self;
}

// Trait for acceptable VectorElements
// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
pub trait VectorComponent:
    Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
    AddAssign + SubAssign + MulAssign + DivAssign +
    PartialEq +
    SqrtDelegate +
    Clone + Copy + Default + Display
    where Self: Sized {

}

// By default VectorComponent isn't implemented! We've implemented it for floating point types inside of component_impls.rs!
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Vector<T: VectorComponent, const COUNT: usize> {
    pub data: [T; COUNT],
}

// Subtypes can declare a better "new" constructor
impl<T: VectorComponent, const COUNT: usize> Vector<T, COUNT> {
    // TODO: Can rust generate a constructor?
    // Like "pub fn new(c1, c2, c3, c4)" ?

    pub fn from_array(d: [T; COUNT]) -> Vector<T, COUNT> {
        Vector { data: d }
    }

    pub fn from_single(d: T) -> Vector<T, COUNT> {
        Vector { data: [d; COUNT] }
    }

    // Sum != Magnitude!
    pub fn sum(&self) -> T {
        let mut sum = T::default();

        self.data.iter().for_each(|x| {
            sum += *x
        });

        sum
    }

    // Magnitude != Sum!
    pub fn magnitude(&self) -> T {
        self.dot(*self).sqrt_delegate()
    }

    pub fn normalize(&self) -> Self {
        return *self / self.magnitude();
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
// Traits
//

//
// Default Trait
//
impl<T: VectorComponent, const COUNT: usize> Default for Vector<T, COUNT> {
    fn default() -> Self {
        let d: [T; COUNT] = [T::default(); COUNT];
        Self { data: d }
    }
}

//
// Indexing Traits
//
impl<T: VectorComponent, const COUNT: usize> Index<usize> for Vector<T, COUNT> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return if index < self.data.len() {
            &self.data[index]
        } else {
            panic!("Index out of range!")
        }
    }
}

impl<T: VectorComponent, const COUNT: usize> IndexMut<usize> for Vector<T, COUNT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return if index < self.data.len() {
            &mut self.data[index]
        } else {
            panic!("Index out of range!")
        }
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
macro_rules! vector_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: VectorComponent, const COUNT: usize> $op<Vector<T, COUNT>> for Vector<T, COUNT> {
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

vector_op!(Add, add, +=);
vector_op!(Sub, sub, -=);
vector_op!(Mul, mul, *=);
vector_op!(Div, div, /=);

//
// Vector Comparison Traits
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