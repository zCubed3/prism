#![allow(unused)]
#![allow(dead_code)]

use super::component::Component;

use std::ops::*;
use std::cmp::*;
use std::fmt::*;

/// [Matrix] and [Vector] are very closely related!
/// Because of this, [Matrix] provides behavior to work with [Vector] types!
///
/// The underlying implementation of a [Matrix] is similar to a [Vector] except a 2D array instead of a 1D array
///
/// # Note:
///     Generic [Matrix] currently lacks determinant() and inverse(), you'll find it on the common types instead!
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Matrix<T: Component, const WIDTH: usize, const HEIGHT: usize> {
    /// The underlying array of the matrix, the matrix dereferences into this array
    pub data: [[T; WIDTH]; HEIGHT],
}

impl<T: Component, const WIDTH: usize, const HEIGHT: usize> Matrix<T, WIDTH, HEIGHT> {
    pub fn from_array(array: [[T; WIDTH]; HEIGHT]) -> Self {
        Self { data: array }
    }

    /// Provides an identity matrix (this works best with evenly shaped [Matrix] types!)
    pub fn identity() -> Self {
        let mut array = [[T::default(); WIDTH]; HEIGHT];

        // Only diagonals are populated, therefore X and Y are the same!
        // We use WIDTH as our frame of reference
        for c in 0 .. WIDTH {
            // Depth check
            if c >= HEIGHT {
                break;
            }

            array[c][c] = T::get_one();
        }

        Self { data: array }
    }

    /// Transposes the matrix (for oddly shaped [Matrix] types, it will flip [WIDTH] and [HEIGHT]!)
    pub fn transpose(&self) -> Matrix<T, HEIGHT, WIDTH> {
        let mut m = Matrix::<T, HEIGHT, WIDTH>::default();

        for y in 0 .. WIDTH {
            for x in 0 .. HEIGHT {
                m[x][y] = self[y][x];
            }
        }

        m
    }
}

//
// Deref
//
impl<T: Component, const WIDTH: usize, const HEIGHT: usize> Deref for Matrix<T, WIDTH, HEIGHT> {
    type Target = [[T; WIDTH]; HEIGHT];

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T: Component, const WIDTH: usize, const HEIGHT: usize> DerefMut for Matrix<T, WIDTH, HEIGHT> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}

//
// Formatters
//
impl<T: Component, const WIDTH: usize, const HEIGHT: usize> Display for Matrix<T, WIDTH, HEIGHT> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0 .. HEIGHT {
            if y != 0 {
                writeln!(f).expect("Failed to write!");
            }

            write!(f, "[").expect("Failed to write!");

            for x in 0 .. WIDTH {
                write!(f, "{}", self[y][x]).expect("Failed to write!");

                if x != WIDTH - 1 {
                    write!(f, ", ").expect("Failed to write!");
                }
            }

            write!(f, "]").expect("Failed to write!");
        }

        Ok(())
    }
}

//
// Default
//
impl<T: Component, const WIDTH: usize, const HEIGHT: usize> Default for Matrix<T, WIDTH, HEIGHT> {
    fn default() -> Self {
        Self { data: [[T::default(); WIDTH]; HEIGHT] }
    }
}

//
// Math
//
macro_rules! component_op_assign {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: Component, const WIDTH: usize, const HEIGHT: usize> $op<T> for Matrix<T, WIDTH, HEIGHT> {
            fn $func(&mut self, rhs: T) {
                for y in 0 .. HEIGHT {
                    for x in 0 .. WIDTH {
                        self[x][y] $call rhs;
                    }
                }
            }
        }
    };
}

macro_rules! component_op {
    ($op:ident, $func:ident, $call:tt) => {
        impl<T: Component, const WIDTH: usize, const HEIGHT: usize> $op<T> for Matrix<T, WIDTH, HEIGHT> {
            type Output = Self;

            fn $func(self, rhs: T) -> Self::Output {
                let mut prod = self;

                for y in 0 .. HEIGHT {
                    for x in 0 .. WIDTH {
                        prod[x][y] $call rhs;
                    }
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
// Common matrix types
//

/// Contains commonly used [Matrix] aliases with additional implementations for ease of use
///
/// Implementations of Matrix inverse are from https://github.com/g-truc/glm/blob/master/glm/detail/func_matrix.inl
pub mod common {
    use crate::math::vector::Vector;
    use super::*;

    /// Matrix 2x2
    pub type Matrix2x2 = Matrix<f32, 2, 2>;

    impl<T: Component> Matrix<T, 2, 2> {
        #[inline]
        pub fn determinant(&self) -> T {
            self[0][0] * self[1][1] - self[0][1] * self[1][0]
        }

        pub fn inverse(&self) -> Self {
            let mut i = Self::default();
            let d = T::get_one() / self.determinant();

            i[0][0] = self[1][1];
            i[0][1] = -self[0][1];
            i[1][0] = -self[1][0];
            i[1][1] = self[0][0];

            i * d
        }
    }

    /// Matrix 3x3
    pub type Matrix3x3 = Matrix<f32, 3, 3>;

    impl<T: Component> Matrix<T, 3, 3> {
        #[inline]
        pub fn determinant(&self) -> T {
            self[0][0] * (self[1][1] * self[2][2] - self[2][1] * self[1][2]) -
            self[1][0] * (self[0][1] * self[2][2] - self[2][1] * self[0][2]) +
            self[2][0] * (self[0][1] * self[1][2] - self[1][1] * self[0][2])
        }

        pub fn inverse(&self) -> Self {
            let mut i = Self::default();
            let d = T::get_one() / self.determinant();

            i[0][0] = (self[1][1] * self[2][2] - self[2][1] * self[1][2]) * d;
            i[1][0] = -(self[1][0] * self[2][2] - self[2][0] * self[1][2]) * d;
            i[2][0] = (self[1][0] * self[2][1] - self[2][0] * self[1][1]) * d;
            i[0][1] = -(self[0][1] * self[2][2] - self[2][1] * self[0][2]) * d;
            i[1][1] = (self[0][0] * self[2][2] - self[2][0] * self[0][2]) * d;
            i[2][1] = -(self[0][0] * self[2][1] - self[2][0] * self[0][1]) * d;
            i[0][2] = (self[0][1] * self[1][2] - self[1][1] * self[0][2]) * d;
            i[1][2] = -(self[0][0] * self[1][2] - self[1][0] * self[0][2]) * d;
            i[2][2] = (self[0][0] * self[1][1] - self[1][0] * self[0][1]) * d;

            i
        }
    }

    /// Matrix 4x4
    pub type Matrix4x4 = Matrix<f32, 4, 4>;

    impl<T: Component> Matrix<T, 4, 4> {
        pub fn from_vectors(r0: Vector<T, 4>, r1: Vector<T, 4>, r2: Vector<T, 4>, r3: Vector<T, 4>) -> Self {
            Self { data: [*r0, *r1, *r2, *r3] }
        }

        pub fn inverse(&self) -> Self {
            let coef00 = self[2][2] * self[3][3] - self[3][2] * self[2][3];
            let coef02 = self[1][2] * self[3][3] - self[3][2] * self[1][3];
            let coef03 = self[1][2] * self[2][3] - self[2][2] * self[1][3];

            let coef04 = self[2][1] * self[3][3] - self[3][1] * self[2][3];
            let coef06 = self[1][1] * self[3][3] - self[3][1] * self[1][3];
            let coef07 = self[1][1] * self[2][3] - self[2][1] * self[1][3];

            let coef08 = self[2][1] * self[3][2] - self[3][1] * self[2][2];
            let coef10 = self[1][1] * self[3][2] - self[3][1] * self[1][2];
            let coef11 = self[1][1] * self[2][2] - self[2][1] * self[1][2];

            let coef12 = self[2][0] * self[3][3] - self[3][0] * self[2][3];
            let coef14 = self[1][0] * self[3][3] - self[3][0] * self[1][3];
            let coef15 = self[1][0] * self[2][3] - self[2][0] * self[1][3];

            let coef16 = self[2][0] * self[3][2] - self[3][0] * self[2][2];
            let coef18 = self[1][0] * self[3][2] - self[3][0] * self[1][2];
            let coef19 = self[1][0] * self[2][2] - self[2][0] * self[1][2];

            let coef20 = self[2][0] * self[3][1] - self[3][0] * self[2][1];
            let coef22 = self[1][0] * self[3][1] - self[3][0] * self[1][1];
            let coef23 = self[1][0] * self[2][1] - self[2][0] * self[1][1];

            let fac0 = Vector::<T, 4>::new(coef00, coef00, coef02, coef03);
            let fac1 = Vector::<T, 4>::new(coef04, coef04, coef06, coef07);
            let fac2 = Vector::<T, 4>::new(coef08, coef08, coef10, coef11);
            let fac3 = Vector::<T, 4>::new(coef12, coef12, coef14, coef15);
            let fac4 = Vector::<T, 4>::new(coef16, coef16, coef18, coef19);
            let fac5 = Vector::<T, 4>::new(coef20, coef20, coef22, coef23);

            let vec0 = Vector::<T, 4>::new(self[1][0], self[0][0], self[0][0], self[0][0]);
            let vec1 = Vector::<T, 4>::new(self[1][1], self[0][1], self[0][1], self[0][1]);
            let vec2 = Vector::<T, 4>::new(self[1][2], self[0][2], self[0][2], self[0][2]);
            let vec3 = Vector::<T, 4>::new(self[1][3], self[0][3], self[0][3], self[0][3]);

            let inv0 = (vec1 * fac0 - vec2 * fac1 + vec3 * fac2);
            let inv1 = (vec0 * fac0 - vec2 * fac3 + vec3 * fac4);
            let inv2 = (vec0 * fac1 - vec1 * fac3 + vec3 * fac5);
            let inv3 = (vec0 * fac2 - vec1 * fac4 + vec2 * fac5);

            let one = T::get_one();

            let sign_a = Vector::<T, 4>::new(one, -one, one, -one);
            let sign_b = -sign_a;

            let mut i = Self::from_vectors(inv0 * sign_a, inv1 * sign_b, inv2 * sign_a, inv3 * sign_b);
            let r0 = Vector::<T, 4>::new(i[0][0], i[1][0], i[2][0], i[3][0]);

            let dot0 = Vector::from_array(self[0]) * r0;
            let dot1 = (dot0[0] + dot0[1]) + (dot0[2] + dot0[3]);

            let d = one / dot1;

            i * d
        }

        pub fn perspective(fov_y: T, near_cull: T, far_cull: T, aspect: T) -> Self {
            let zero = T::default();
            let one: T = T::get_one();
            let two: T = one + one;

            let rad: T = (fov_y / two) * T::deg_to_rad();

            let y_scale = one / rad.tan_delegate();
            let x_scale = y_scale / aspect;
            let comp = near_cull - far_cull;

            let mut m = Self::default();

            m[0] = [x_scale, zero, zero, zero];
            m[1] = [zero, y_scale, zero, zero];
            m[2] = [zero, zero, (far_cull + near_cull) / comp, -one];
            m[3] = [zero, zero, two * far_cull * near_cull / comp, zero];

            m
        }
    }
}