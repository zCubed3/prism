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

/// Required trait for matrix components!
///
/// Because of [Matrix::identity()], it is necessary to get 0 and 1 of the given component type!
pub trait GetOne {
    fn get_one() -> Self;
}

/// Required trait for matrix components!
///
/// Because of [Matrix4x4::perspective()], it is necessary to get the tangent of a given component type
pub trait TanDelegate {
    fn tan_delegate(&self) -> Self;
}

/// Required trait for operations requiring conversions!
pub trait Constants {
    fn rad_to_deg() -> Self;
    fn deg_to_rad() -> Self;

    fn pi() -> Self;
}


// https://www.worthe-it.co.za/blog/2017-01-15-aliasing-traits-in-rust.html
/// Strict trait for constraining what types can be used as vector components
///
/// This trait is already implemented for [f32] and [f64]
pub trait Component:
Add<Output=Self> + Sub<Output=Self> + Mul<Output=Self> + Div<Output=Self> +
AddAssign + SubAssign + MulAssign + DivAssign +
Neg<Output=Self> +
PartialEq +
SqrtDelegate + GetOne + TanDelegate + Constants +
Clone + Copy + Default + Display
    where Self: Sized {

}

//
// Components for float types
//

// F32
impl Component for f32 {}

impl SqrtDelegate for f32 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}

impl GetOne for f32 {
    fn get_one() -> Self {
        1f32
    }
}

impl TanDelegate for f32 {
    fn tan_delegate(&self) -> Self {
        self.tan()
    }
}

impl Constants for f32 {
    fn rad_to_deg() -> Self {
        57.2957795131f32
    }

    fn deg_to_rad() -> Self {
        0.01745329251f32
    }

    fn pi() -> Self {
        std::f32::consts::PI
    }
}

// F64
impl Component for f64 {}

impl SqrtDelegate for f64 {
    fn sqrt_delegate(&self) -> Self {
        self.sqrt()
    }
}

impl GetOne for f64 {
    fn get_one() -> Self {
        1f64
    }
}

impl TanDelegate for f64 {
    fn tan_delegate(&self) -> Self {
        self.tan()
    }
}

impl Constants for f64 {
    fn rad_to_deg() -> Self {
        57.2957795131f64
    }

    fn deg_to_rad() -> Self {
        0.01745329251f64
    }

    fn pi() -> Self {
        std::f64::consts::PI
    }
}